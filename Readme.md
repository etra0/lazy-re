# Lazy-RE

A simple proc macro for the lazy reverse engineers.
It basically creates the padding for you.

```rust
#[repr(C, packed)]
#[derive(LazyRe)]
#[lazy_re]
struct Lights {
    #[offset = 0x10]
    x: f32,
    y: f32,
    z: f32
}

#[repr(C, packed)]
#[derive(LazyRe)]
#[lazy_re]
struct PlayerEntity {
    #[offset = 0x48]
    light: Lights,

    #[offset = 0x90]
    player_x: f32,
    player_y: f32,
    player_z: f32,
}
```

That would create the padding for the `Light` struct at the beginning, i.e.
the `x` field will be at the offset `0x10`, and the rest is filled with `[u8;
0x10]`.

Similarly, the PlayerEntity will have padding until the `Light` struct, and
then it'll pad between the light and the player position, doing the math for
you.
