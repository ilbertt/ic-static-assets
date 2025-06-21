use ic_cdk::{init, post_upgrade, query};
use ic_http_certification::{HttpRequest, HttpResponse};
use ic_static_assets::include_assets;

// This is the important line that embeds the frontend assets into the backend!
include_assets!("$CARGO_MANIFEST_DIR/../../src/frontend/dist");

#[init]
fn init() {
    certify_assets();
}

#[post_upgrade]
fn post_upgrade() {
    certify_assets();
}

#[query]
fn http_request(req: HttpRequest<'static>) -> HttpResponse<'static> {
    http_request_handler(&req)
}
