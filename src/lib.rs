#![cfg_attr(all(feature = "nightly", feature = "unstable"), feature(decl_macro))]

use proc_macro::TokenStream;

/// Examples:
///
/// `mask!(..9)`
/// `mask!(2..=9)`
///
/// Errors:
///
/// const _: u32 = bitmask!(31..); // implicit `to` limit requares to set size suffix for `from` limit
/// const _: u32 = bitmask!(2o32..31); // invalid suffix `o32` for integer literal
#[proc_macro]
pub fn bitmask(input: TokenStream) -> TokenStream {
    bitmask::proc_macro(input)
}

mod bitmask;
