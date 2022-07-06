//! # Lazy RE
//! Sometimes we're lazy and we don't need to fully reverse engineer a struct, so we can omit some
//! fields we're not interested in.
//!
//! With this library, you can generate padding without the need of doing mental math every time
//! you need to change your struct, so you won't have to keep track of the padding in your head,
//! this proc macro will generate it for you!
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::parse::{Parse, Parser};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, LitInt, Token};

struct Offset(usize);

mod keyword {
    syn::custom_keyword!(offset);
}

fn get_fields<'a>(
    ast: &'a mut syn::Data,
    ident: &'_ syn::Ident,
) -> syn::Result<&'a mut FieldsNamed> {
    match ast {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref mut fields),
            ..
        }) => Ok(fields),
        _ => Err(syn::Error::new(ident.span(), "Expected named struct")),
    }
}

/// This macro is in charge of generating the Debug implementation for the struct and the `::new`
/// method. It is optional to include.
///
/// The implementation for the Debug trait will omit all the padding fields.
#[proc_macro_derive(LazyRe)]
pub fn derive_helper_attr(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match derive_helper_attr_impl(ast) {
        Ok(res) => res,
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_helper_attr_impl(mut ast: DeriveInput) -> syn::Result<TokenStream> {
    let fields = &mut get_fields(&mut ast.data, &ast.ident)?.named;

    let ident_string = ast.ident.to_string();
    let ident = ast.ident;
    // Safety:
    // We are sure we're reading things that *actually* exist in memory.
    let fields_names = fields
        .iter()
        .flat_map(|x| &x.ident)
        .filter(|x| !x.to_string().starts_with("__pad")) // This is ugly, I wish we didn't need to do this.
        .map(|ident| {
            let ident_string = ident.to_string();
            return quote! { .field(#ident_string,
                   unsafe { &std::ptr::read_unaligned(std::ptr::addr_of!(self.#ident)) }) };
        });

    let output = quote! {
        impl std::fmt::Debug for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return f.debug_struct(#ident_string)
                    #( #fields_names )*
                    .finish();
            }
        }
    };

    Ok(output.into())
}

impl Parse for Offset {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::offset>()?;
        input.parse::<Token![=]>()?;
        let val: LitInt = input.parse()?;

        Ok(Offset(val.base10_parse()?))
    }
}

fn lazy_re_impl(mut ast: DeriveInput) -> syn::Result<TokenStream> {
    let mut all_fields = Vec::new();
    let mut current_ix: usize = 0;
    let mut is_repr_c_packed = false;

    let fields = &mut get_fields(&mut ast.data, &ast.ident)?.named;

    // We need to check if the struct we're working with implements #[repr(C, packed)]. That's the
    // only way we can guarantee the sizes correspond to what we're declaring, since a struct with
    // offset could have some sort of padding which could make bugs harder to track down. The main
    // disadvantage is that we cannot have pointers to everything because misalignment could
    // happen.
    for attr in ast.attrs.iter() {
        let (path, nested) = match attr.parse_meta()? {
            syn::Meta::List(syn::MetaList { path, nested, .. }) => (path, nested),
            _ => continue,
        };

        if !path.is_ident("repr") {
            continue;
        }

        let nested_names = nested
            .iter()
            .map(|x| match x {
                syn::NestedMeta::Meta(m) => m.path().get_ident().unwrap().to_string(),
                _ => panic!("This shouldn't be on a repr C"),
            })
            .collect::<HashSet<_>>();

        is_repr_c_packed = nested_names.contains("C") && nested_names.contains("packed");
    }

    if !is_repr_c_packed {
        return Err(syn::Error::new(
            ast.ident.span(),
            "The struct does not have the attribute #[repr(C, packed)]",
        ));
    }

    let local_fields = std::mem::replace(fields, syn::punctuated::Punctuated::new());
    for mut field in IntoIterator::into_iter(local_fields) {
        let mut offs = None;
        // We need to check the attribute offset is actually present on the struct.
        let mut ix_to_remove = None;
        for (i, attr) in field.attrs.iter().enumerate() {
            if !attr.path.is_ident("lazy_re") {
                continue;
            }

            offs = Some(attr.parse_args::<Offset>()?.0);
            ix_to_remove = Some(i);
        }

        if offs.is_none() {
            all_fields.push(field);
            continue;
        }

        // ix_to_remove is Some if offs is some, So we can be sure this would never fail.
        field.attrs.remove(ix_to_remove.unwrap());
        let offs = offs.unwrap();

        let new_ident = format_ident!("__pad{:03}", current_ix);
        current_ix += 1;
        let all_fields_ty = all_fields.iter().map(|field| &field.ty);
        let field_to_add = syn::Field::parse_named
            .parse2(quote! {  #new_ident: [u8; #offs - (0 #(+ std::mem::size_of::<#all_fields_ty>())*)]})
            .unwrap();

        all_fields.push(field_to_add);
        all_fields.push(field);
    }

    fields.extend(all_fields.drain(..));

    Ok(quote! { #ast }.into())
}

/// This proc macro will generate padding fields for your struct every time you have a struct that
/// has fields with the macro.
///
/// # Example
///
/// ```
/// use lazy_re::lazy_re;
/// #[lazy_re]
/// #[repr(C, packed)]
/// pub struct Foo {
///     #[lazy_re(offset = 0x42)]
///     pub foo: usize
/// }
/// ```
///
/// This struct now will be expanded to a struct with two fields and its respective padding:
///
/// ```
/// use lazy_re::lazy_re;
/// #[lazy_re]
/// #[repr(C, packed)]
/// pub struct Foo {
///     __pad000: [u8; 0x42],
///     pub foo: usize
/// }
/// ```
///
/// The utility of this macro is when you're reverse engineering something and you're only
/// interested in some fields of a big struct, you can use this macro to cast raw pointers.
#[proc_macro_attribute]
pub fn lazy_re(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match lazy_re_impl(ast) {
        Ok(res) => res,
        Err(e) => e.to_compile_error().into(),
    }
}
