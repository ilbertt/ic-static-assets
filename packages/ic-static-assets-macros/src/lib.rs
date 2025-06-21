use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[cfg(doc)]
use include_dir::{Dir, include_dir};

/// Requires you to import the `include_dir` elements:
///
/// ```rust
/// use include_dir::{Dir, include_dir};
/// use ic_static_assets_macros::include_assets;
///
/// include_assets!("$CARGO_MANIFEST_DIR");
/// ```
#[proc_macro]
pub fn include_assets(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::LitStr);
    quote! {
        static ASSETS_DIR: include_dir::Dir<'static> = include_dir::include_dir!(#input);
    }
    .into()
}
