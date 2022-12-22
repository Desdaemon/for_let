#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    Expr, Pat, Result, Token,
};

struct ForPattern(TokenStream);

impl Parse for ForPattern {
    #[inline]
    fn parse(input: ParseStream) -> Result<Self> {
        let pat = Pat::parse(input)?;
        let mut guard = None;
        if input.peek(Token![if]) {
            <Token![if]>::parse(input)?;
            let cond = Expr::parse_without_eager_brace(input)?;
            guard = Some(quote_spanned!(cond.span() => if #cond));
        }

        <Token![in]>::parse(input)?;
        let iter = Expr::parse_without_eager_brace(input)?;

        let blk;
        braced!(blk in input);

        let blk = proc_macro2::TokenStream::parse(&blk)?;

        Ok(Self(
            quote! {
                for __el in #iter {
                    match __el {
                        #pat #guard => {#blk}
                        _ => {}
                    }
                }
            }
            .into(),
        ))
    }
}

/// Allows a single [match arm](https://doc.rust-lang.org/reference/expressions/match-expr.html#match-guards)
/// in the iteratee of a for-loop.
///
/// Examples:
/// ```
/// # use for_let::for_let;
/// let foos = [None, Some("foo"), None, Some("bar")];
///
/// // All patterns are valid, with an optional guard
/// for_let!(Some(foo) if foo == "foo" in foos {
///     println!("Got a '{}'!", foo);
/// });
///
/// // Conditional patterns
/// for_let!(Some("foo" | "bar") in foos {
///     println!("Got a 'foo' or a 'bar'!");
/// });
///
/// // Binding patterns
/// for_let!(Some(bar @ "bar") in foos {
///     assert_eq!(bar, "bar");
/// });
/// ```
#[proc_macro]
pub fn for_let(ts: TokenStream) -> TokenStream {
    let ForPattern(ts) = parse_macro_input!(ts as ForPattern);
    ts
}
