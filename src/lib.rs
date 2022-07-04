use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parser, Parse};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Token, LitInt};

struct Offset(usize);

mod keyword {
    syn::custom_keyword!(offset);
}

#[proc_macro_derive(LazyRe, attributes(offset))]
pub fn derive_helper_attr(_input: TokenStream) -> TokenStream {
    // TODO: impl Debug, new.
    TokenStream::new()
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

    let fields = match &mut ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref mut fields),
            ..
        }) => &mut fields.named,
        _ => {
            return Err(syn::Error::new(ast.ident.span(), "Expected named struct"));
        }
    };

    // This is too cumbersome, but I don't know any other way of easily checking if the struct is
    // actually a repr of C and it's packed.
    for attr in ast.attrs.iter() {
        let (path, nested) = match attr.parse_meta()? {
            syn::Meta::List(syn::MetaList { path, nested, .. }) => (Some(path), Some(nested)),
            _ => (None, None),
        };

        if path.is_none() {
            continue;
        }

        let path = path.unwrap();
        if path.get_ident().unwrap() != "repr" {
            continue;
        }

        let nested = nested.unwrap();
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
        ))
    }

    for field in fields.iter_mut() {
        let mut offs = None;
        // We need to check the attribute offset is actually present on the struct.
        // TODO: Maybe omit using the derive and just make everything in lazy_re.
        let mut ix_to_remove = None;
        for (i, attr) in field.attrs.iter().enumerate() {
            if !attr.path.is_ident("lazy_re") {
                continue;
            }

            offs = Some(attr.parse_args::<Offset>()?.0);
            ix_to_remove = Some(i);
        }

        if offs.is_none() {
            all_fields.push(field.clone());
            continue;
        }

        field.attrs.remove(ix_to_remove.unwrap());
        let offs = offs.unwrap();
        let new_ident = format_ident!("__pad{:03}", current_ix);
        current_ix += 1;
        let all_fields_ty = all_fields.iter().map(|field| &field.ty);
        let field_to_add = if all_fields.len() > 0 {
            syn::Field::parse_named
            .parse2(quote! {  #new_ident: [u8; #offs - (#(std::mem::size_of::<#all_fields_ty>())+*)]})
            .unwrap()
        } else {
            syn::Field::parse_named
                .parse2(quote! {  #new_ident: [u8; #offs]})
                .unwrap()
        };

        all_fields.push(field_to_add);
        all_fields.push(field.clone());
    }

    // There's probably an easier way of doing this, but for now I'm OK with this.
    fields.clear();
    all_fields.drain(..).for_each(|x| fields.push(x));

    Ok(quote! { #ast }.into())
}

#[proc_macro_attribute]
pub fn lazy_re(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match lazy_re_impl(ast) {
        Ok(res) => res,
        Err(e) => e.to_compile_error().into()
    }
}
