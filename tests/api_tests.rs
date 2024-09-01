// src/tests/api_tests.rs

use actix_web::{test, App};
use crate::api::routes::{configure_routes, init_services};

#[actix_web::test]
async fn test_create_block() {
    let app = test::init_service(App::new().configure(init_services).configure(configure_routes)).await;
    let req = test::TestRequest::post().uri("/block").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_consensus_status() {
    let app = test::init_service(App::new().configure(init_services).configure(configure_routes)).await;
    let req = test::TestRequest::get().uri("/consensus").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
