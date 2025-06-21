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
pub use certification::{
    certify_all_assets, certify_all_assets_with_internal_router, serve_assets,
    serve_assets_with_internal_router,
};
#[doc(hidden)]
pub use ic_asset_certification::AssetRouter;
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
/// - a `certify_assets_hook` function that can be used to certify the assets.
/// - a `http_request_handler` function that can be used to serve the assets.
///
/// # Arguments
///
/// * `$path` - A string literal representing the path to the directory to include
/// * `$certify_fn` - (Optional) A function with signature `fn certify_assets<'path>(assets: Vec<Asset<'static, 'path>>, asset_configs: Vec<AssetConfig>) -> Result<Hash, AssetCertificationError>`
/// * `$serve_fn` - (Optional) A function with signature `fn serve_assets(root_hash: Vec<u8>, req: &HttpRequest<'static>) -> HttpResponse<'static>`
///
/// # Basic Example
/// See the [example](index.html#example) in the root of the crate.
///
/// # Example with custom router functions
/// ```rust
/// use ic_static_assets::include_assets;
/// use ic_asset_certification::{Asset, AssetCertificationError, AssetConfig, AssetRouter};
/// use ic_http_certification::{Hash, HttpRequest, HttpResponse};
/// use std::cell::RefCell;
/// use std::thread_local;
///
/// thread_local! {
///     static ROUTER: RefCell<AssetRouter<'static>> = RefCell::new(AssetRouter::new());
/// }
///
/// fn certify_my_assets(assets: Vec<Asset<'static, '_>>, asset_configs: Vec<AssetConfig>) -> Result<Hash, AssetCertificationError> {
///     ROUTER.with_borrow_mut(|router| {
///         router.certify_assets(assets, asset_configs)?;
///         Ok(router.root_hash())
///     })
/// }
///
/// fn serve_my_assets(root_hash: Vec<u8>, req: &HttpRequest<'static>) -> HttpResponse<'static> {
///     ROUTER.with_borrow(|router| {
///         router.serve_asset(&root_hash, req).expect("Failed to serve asset")
///     })
/// }
///
/// include_assets!("$CARGO_MANIFEST_DIR", certify_my_assets, serve_my_assets);
/// ```
#[macro_export]
macro_rules! include_assets {
    ($path:expr) => {
        use $crate::include_dir;

        $crate::ic_static_assets_macros::include_assets!($path);

        #[allow(dead_code)]
        fn certify_assets_hook() {
            $crate::certify_all_assets_with_internal_router(&ASSETS_DIR);
        }

        #[allow(dead_code)]
        fn http_request_handler(
            req: &$crate::HttpRequest<'static>,
        ) -> $crate::HttpResponse<'static> {
            $crate::serve_assets_with_internal_router(&req)
        }
    };
    ($path:expr, $certify_fn:expr, $serve_fn:expr) => {
        use $crate::include_dir;

        $crate::ic_static_assets_macros::include_assets!($path);

        #[allow(dead_code)]
        fn certify_assets_hook() {
            $crate::certify_all_assets(&ASSETS_DIR, $certify_fn);
        }

        #[allow(dead_code)]
        fn http_request_handler(
            req: &$crate::HttpRequest<'static>,
        ) -> $crate::HttpResponse<'static> {
            $crate::serve_assets(&req, $serve_fn)
        }
    };
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use ic_asset_certification::{Asset, AssetCertificationError, AssetConfig};
    use ic_http_certification::{Hash, HttpRequest, HttpResponse};

    use super::*;

    #[test]
    fn test_include_assets_macro() {
        // This test demonstrates the macro usage
        // In a real scenario, you would have actual assets in your project
        include_assets!("$CARGO_MANIFEST_DIR");

        // Verify that ASSETS is accessible and loads the files properly
        assert!(ASSETS_DIR.get_file("Cargo.toml").is_some());
    }

    #[test]
    fn test_include_assets_macro_with_router() {
        // This test demonstrates the macro usage with custom router
        thread_local! {
            static ROUTER: RefCell<AssetRouter<'static>> = RefCell::new(AssetRouter::new());
        }

        fn certify_my_assets(
            assets: Vec<Asset<'static, '_>>,
            asset_configs: Vec<AssetConfig>,
        ) -> Result<Hash, AssetCertificationError> {
            ROUTER.with_borrow_mut(|router| {
                router.certify_assets(assets, asset_configs)?;
                Ok(router.root_hash())
            })
        }

        fn serve_my_assets(
            root_hash: Vec<u8>,
            req: &HttpRequest<'static>,
        ) -> HttpResponse<'static> {
            ROUTER.with_borrow(|router| {
                router
                    .serve_asset(&root_hash, req)
                    .expect("Failed to serve asset")
            })
        }

        include_assets!("$CARGO_MANIFEST_DIR", certify_my_assets, serve_my_assets);

        // Verify that ASSETS is accessible and loads the files properly
        assert!(ASSETS_DIR.get_file("Cargo.toml").is_some());
    }
}
