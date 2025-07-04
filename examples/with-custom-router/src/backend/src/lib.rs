use std::cell::RefCell;

use ic_asset_certification::{Asset, AssetCertificationError, AssetConfig, AssetRouter};
use ic_cdk::{init, post_upgrade, query};
use ic_http_certification::{Hash, HttpRequest, HttpResponse};
use ic_static_assets::include_assets;

thread_local! {
    static ASSET_ROUTER: RefCell<AssetRouter<'static>> = RefCell::new(AssetRouter::default());
}

fn certify_my_assets(
    assets: Vec<Asset<'static, '_>>,
    asset_configs: Vec<AssetConfig>,
) -> Result<Hash, AssetCertificationError> {
    ASSET_ROUTER.with_borrow_mut(|router| {
        router.certify_assets(assets, asset_configs)?;
        Ok(router.root_hash())
    })
}

fn serve_my_assets(root_hash: Vec<u8>, req: &HttpRequest<'static>) -> HttpResponse<'static> {
    ASSET_ROUTER.with_borrow(|router| {
        router
            .serve_asset(&root_hash, req)
            .expect("Failed to serve asset")
    })
}

// This is the important line that embeds the frontend assets into the backend!
include_assets!(
    "$CARGO_MANIFEST_DIR/../frontend/dist",
    certify_my_assets,
    serve_my_assets
);

#[init]
fn init() {
    certify_assets_hook(); // <- generated by the `include_assets` macro, no need to import it
}

#[post_upgrade]
fn post_upgrade() {
    certify_assets_hook();
}

#[query]
fn http_request(req: HttpRequest<'static>) -> HttpResponse<'static> {
    http_request_handler(&req) // <- generated by the `include_assets` macro, no need to import it
}
