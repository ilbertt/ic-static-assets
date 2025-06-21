use std::cell::RefCell;

use ic_asset_certification::{Asset, AssetConfig, AssetEncoding, AssetFallbackConfig, AssetRouter};
use ic_http_certification::{HeaderField, HttpRequest, HttpResponse, StatusCode};
use include_dir::Dir;

const CACHE_CONTROL_HEADER_NAME: &str = "cache-control";
const ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_NAME: &str = "access-control-allow-origin";
const IMMUTABLE_ASSET_CACHE_CONTROL: &str = "public, max-age=31536000, immutable";
const NO_CACHE_ASSET_CACHE_CONTROL: &str = "public, no-cache, no-store";

const WELL_KNOWN_PATH: &str = "/.well-known";
const II_ALTERNATIVE_ORIGINS_FILE_NAME: &str = "ii-alternative-origins";

#[derive(Default)]
struct HttpAssetState<'a> {
    router: AssetRouter<'a>,
}

thread_local! {
    static STATE: RefCell<HttpAssetState<'static>> = RefCell::new(HttpAssetState::default());
}

fn get_asset_headers(additional_headers: Vec<HeaderField>) -> Vec<HeaderField> {
    // set up the default headers and include additional headers provided by the caller
    let mut headers = vec![
        ("strict-transport-security".to_string(), "max-age=31536000; includeSubDomains".to_string()),
        ("x-frame-options".to_string(), "DENY".to_string()),
        ("x-content-type-options".to_string(), "nosniff".to_string()),
        ("content-security-policy".to_string(), "default-src 'self';script-src 'self' 'unsafe-inline' 'unsafe-eval';connect-src 'self' http://localhost:* https://icp0.io https://*.icp0.io https://icp-api.io https://fastly.jsdelivr.net;img-src 'self' https://*.icp0.io data: blob:;style-src * 'unsafe-inline';style-src-elem * 'unsafe-inline';font-src *;object-src 'none';media-src 'self' data:;base-uri 'self';frame-ancestors 'none';form-action 'self';upgrade-insecure-requests".to_string()),
        ("referrer-policy".to_string(), "same-origin".to_string()),
        ("permissions-policy".to_string(), "accelerometer=(), ambient-light-sensor=(), autoplay=(), battery=(), camera=(self), cross-origin-isolated=(), display-capture=(), document-domain=(), encrypted-media=(), execution-while-not-rendered=(), execution-while-out-of-viewport=(), fullscreen=(), geolocation=(), gyroscope=(), keyboard-map=(), magnetometer=(), microphone=(), midi=(), navigation-override=(), payment=(), picture-in-picture=(), publickey-credentials-get=(), screen-wake-lock=(), sync-xhr=(), usb=(), web-share=(), xr-spatial-tracking=(), clipboard-read=(), clipboard-write=(self), gamepad=(), speaker-selection=(), conversion-measurement=(), focus-without-user-activation=(), hid=(), idle-detection=(), interest-cohort=(), serial=(), sync-script=(), trust-token-redemption=(), window-placement=(), vertical-scroll=()".to_string()),
        ("x-xss-protection".to_string(), "1; mode=block".to_string()),
    ];
    headers.extend(additional_headers);

    headers
}

fn collect_assets<'content, 'path>(
    dir: &'content Dir<'path>,
    assets: &mut Vec<Asset<'content, 'path>>,
) {
    for file in dir.files() {
        let file_path = file.path();
        if file_path.ends_with(".gitkeep") {
            // do not expose .gitkeep files
            continue;
        }
        assets.push(Asset::new(file_path.to_string_lossy(), file.contents()));
    }

    for dir in dir.dirs() {
        collect_assets(dir, assets);
    }
}

pub fn certify_all_assets(assets_dir: &'static Dir<'static>) {
    // 1. Define the asset certification configurations.
    let encodings = vec![
        AssetEncoding::Brotli.default_config(),
        AssetEncoding::Gzip.default_config(),
    ];

    let canister_id_cookie_header: HeaderField = (
        "set-cookie".to_string(),
        // can be a session cookie, as we set it on every request
        format!("canisterId={}", ic_cdk::api::canister_self().to_text()),
    );

    let asset_configs = vec![
        AssetConfig::File {
            path: "index.html".to_string(),
            content_type: Some("text/html".to_string()),
            headers: get_asset_headers(vec![
                (
                    CACHE_CONTROL_HEADER_NAME.to_string(),
                    NO_CACHE_ASSET_CACHE_CONTROL.to_string(),
                ),
                canister_id_cookie_header.clone(),
            ]),
            fallback_for: vec![AssetFallbackConfig {
                scope: "/".to_string(),
                status_code: Some(StatusCode::OK),
            }],
            aliased_by: vec!["/".to_string()],
            encodings: encodings.clone(),
        },
        AssetConfig::Pattern {
            pattern: "**/*.js".to_string(),
            content_type: Some("text/javascript".to_string()),
            headers: get_asset_headers(vec![(
                CACHE_CONTROL_HEADER_NAME.to_string(),
                IMMUTABLE_ASSET_CACHE_CONTROL.to_string(),
            )]),
            encodings: encodings.clone(),
        },
        AssetConfig::Pattern {
            pattern: "**/*.css".to_string(),
            content_type: Some("text/css".to_string()),
            headers: get_asset_headers(vec![(
                CACHE_CONTROL_HEADER_NAME.to_string(),
                IMMUTABLE_ASSET_CACHE_CONTROL.to_string(),
            )]),
            encodings: encodings.clone(),
        },
        AssetConfig::Pattern {
            pattern: "**/*.png".to_string(),
            content_type: Some("image/png".to_string()),
            headers: get_asset_headers(vec![(
                CACHE_CONTROL_HEADER_NAME.to_string(),
                IMMUTABLE_ASSET_CACHE_CONTROL.to_string(),
            )]),
            encodings: encodings.clone(),
        },
        AssetConfig::Pattern {
            pattern: "**/*.svg".to_string(),
            content_type: Some("image/svg+xml".to_string()),
            headers: get_asset_headers(vec![(
                CACHE_CONTROL_HEADER_NAME.to_string(),
                IMMUTABLE_ASSET_CACHE_CONTROL.to_string(),
            )]),
            encodings: encodings.clone(),
        },
        AssetConfig::Pattern {
            pattern: "**/*.txt".to_string(),
            content_type: Some("text/plain".to_string()),
            headers: get_asset_headers(vec![(
                CACHE_CONTROL_HEADER_NAME.to_string(),
                IMMUTABLE_ASSET_CACHE_CONTROL.to_string(),
            )]),
            encodings,
        },
        AssetConfig::Pattern {
            pattern: "**/*.ico".to_string(),
            content_type: Some("image/x-icon".to_string()),
            headers: get_asset_headers(vec![(
                CACHE_CONTROL_HEADER_NAME.to_string(),
                IMMUTABLE_ASSET_CACHE_CONTROL.to_string(),
            )]),
            encodings: vec![],
        },
        AssetConfig::Pattern {
            pattern: "**/*.otf".to_string(),
            content_type: Some("application/x-font-opentype".to_string()),
            headers: get_asset_headers(vec![(
                CACHE_CONTROL_HEADER_NAME.to_string(),
                IMMUTABLE_ASSET_CACHE_CONTROL.to_string(),
            )]),
            encodings: vec![],
        },
        AssetConfig::Pattern {
            pattern: format!("{WELL_KNOWN_PATH}/*"),
            content_type: None,
            headers: well_known_asset_headers(),
            encodings: vec![],
        },
        AssetConfig::File {
            path: format!("{WELL_KNOWN_PATH}/{II_ALTERNATIVE_ORIGINS_FILE_NAME}"),
            content_type: Some("application/json".to_string()),
            headers: well_known_asset_headers(),
            fallback_for: vec![],
            aliased_by: vec![],
            encodings: vec![],
        },
    ];

    // 2. Collect all assets from the frontend build directory.
    let mut assets = Vec::new();
    collect_assets(assets_dir, &mut assets);

    STATE.with_borrow_mut(|state| {
        if let Err(err) = state.router.certify_assets(assets, asset_configs) {
            ic_cdk::trap(format!("Failed to certify assets: {}", err));
        }
        ic_cdk::api::certified_data_set(state.router.root_hash());
    });
}

pub fn serve_assets(request: &HttpRequest<'static>) -> HttpResponse<'static> {
    STATE.with_borrow(|s| {
        let data_certificate =
            ic_cdk::api::data_certificate().expect("Failed to get data certificate");
        s.router
            .serve_asset(&data_certificate, request)
            .expect("Failed to serve asset")
    })
}

fn well_known_asset_headers() -> Vec<HeaderField> {
    get_asset_headers(vec![
        (
            CACHE_CONTROL_HEADER_NAME.to_string(),
            NO_CACHE_ASSET_CACHE_CONTROL.to_string(),
        ),
        (
            ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_NAME.to_string(),
            "*".to_string(),
        ),
    ])
}
