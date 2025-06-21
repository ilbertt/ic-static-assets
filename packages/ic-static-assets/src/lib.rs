//! # `ic-static-assets`
//!
//! This crate provides a macro that embeds the contents of a directory into a static variable.
//! It also provides a function to certify the assets and a function to serve the assets.
//!
//! ## Example
//! ```rust,ignore
#![doc = include_str!("../../../examples/basic/src/backend/src/lib.rs")]
//! ```

mod certification;

#[doc(hidden)]
pub use certification::{certify_all_assets, serve_assets};
#[doc(hidden)]
pub use ic_static_assets_macros;

#[doc(hidden)]
pub use ic_http_certification::{HttpRequest, HttpResponse};
#[doc(hidden)]
pub use include_dir;

/// The main macro that embeds the static assets of a directory into a static variable at compile time.
///
/// This macro generates:
/// - a static `ASSETS_DIR` variable containing the contents of a directory.
/// - a `certify_assets` function that can be used to certify the assets.
/// - a `http_request_handler` function that can be used to serve the assets.
///
/// # Arguments
///
/// * `$path` - A string literal representing the path to the directory to include
///
/// # Example
/// See the [example](index.html#example) in the root of the crate.
#[macro_export]
macro_rules! include_assets {
    ($path:expr) => {
        use $crate::include_dir;

        $crate::ic_static_assets_macros::include_assets!($path);

        #[allow(dead_code)]
        fn certify_assets() {
            $crate::certify_all_assets(&ASSETS_DIR);
        }

        #[allow(dead_code)]
        fn http_request_handler(
            req: &$crate::HttpRequest<'static>,
        ) -> $crate::HttpResponse<'static> {
            $crate::serve_assets(&req)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_include_assets_macro() {
        // This test demonstrates the macro usage
        // In a real scenario, you would have actual assets in your project
        include_assets!("$CARGO_MANIFEST_DIR");

        // Verify that ASSETS is accessible and loads the files properly
        assert!(ASSETS_DIR.get_file("Cargo.toml").is_some());
    }
}
