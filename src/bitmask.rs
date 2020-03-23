use proc_macro::TokenStream;
use quote::quote;
use std::mem::size_of;
use syn::{parse_macro_input, parse_quote, Expr, ExprLit, ExprRange, Lit, LitInt, RangeLimits};

#[cfg(all(feature = "nightly", feature = "unstable"))]
macro quote_bit_size($ty:ty) {{
    let bits = (size_of::<$ty>() * 8) as $ty;
    parse_quote!(#bits)
}}

#[cfg(not(all(feature = "nightly", feature = "unstable")))]
macro_rules! quote_bit_size {
    ($ty:ty) => {{
        let bits = (size_of::<$ty>() * 8) as $ty;
        parse_quote!(#bits)
    }};
}

pub(super) fn proc_macro(input: TokenStream) -> TokenStream {
    let expr_range = parse_macro_input!(input as ExprRange);
    let default_from = parse_quote!(0);
    let default_to = match expr_range.from.as_deref() {
        Some(Expr::Lit(ExprLit {
            lit: Lit::Int(ref lit_int),
            ..
        })) => lit_int_to_bit_size(lit_int),
        _ => None,
    };
    let from = &expr_range.from.unwrap_or(default_from);
    let to = &expr_range
        .to
        .or(default_to)
        .expect("implicit `to` limit requares to set valid size suffix for `from` limit");
    let limits = &expr_range.limits;
    // n equal bits count minus one
    // - `from..to`: (to - from) - 1
    // - `from..=to`: (to - from + 1) - 1
    let n = match limits {
        RangeLimits::HalfOpen(_) => quote!(#to - #from - 1),
        RangeLimits::Closed(_) => quote!(#to - #from),
    };
    let expanded = quote! {
        (((1 << #n) - 1) << 1 | 1) << #from
    };
    TokenStream::from(expanded)
}

fn lit_int_to_bit_size(lit_int: &LitInt) -> Option<Box<Expr>> {
    let suffix = lit_int.suffix();
    match suffix {
        "u8" => Some(quote_bit_size!(u8)),
        "i8" => Some(quote_bit_size!(i8)),
        "u16" => Some(quote_bit_size!(u16)),
        "i16" => Some(quote_bit_size!(i16)),
        "u32" => Some(quote_bit_size!(u32)),
        "i32" => Some(quote_bit_size!(i32)),
        "u64" => Some(quote_bit_size!(u64)),
        "i64" => Some(quote_bit_size!(i64)),
        "isize" => Some(quote_bit_size!(isize)),
        "usize" => Some(quote_bit_size!(usize)),
        _ => None,
    }
}
