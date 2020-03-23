# bitmask

Procedural macros for set bit mask.

```rust
bitflags! {
    struct Fields: u32 {
        const A = bitmask!(..=2);
        const B = bitmask!(2..9);
        const C = bitmask!(9..32);
        const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}
```
