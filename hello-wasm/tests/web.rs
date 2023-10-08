//! Test suite for the Web and headless browsers.

// #![cfg(target_arch = "wasm32")]

// extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use hello_wasm::v2;
use wasm_bindgen_futures::JsFuture;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
async fn post_v1() {
    let v2 = v2::V2::new();

    let m2 = v2::Metric{client_id: 1, auth_token: String::from("auth_test"), datetime: v2::DateTime{year: 2023, millisecond: 21}};
    
    // let resp  js_sys::Promise::resolve(&v2.publish(m2).await);
    let resp = v2.publish(m2).await;

    match resp {
        Ok(data) => {
            let p = js_sys::Promise::resolve(&data);
            let x: JsValue = JsFuture::from(p).await.unwrap();
            // let resp: common::MetricsResponse = x.into_serde().unwrap();
            let resp: common::MetricsResponse = serde_wasm_bindgen::from_value(x).unwrap();
            // println!(">>> test p: {:?}", x);
            assert_eq!(resp.status, "ok");
            assert_eq!(resp.count, 1);
        },
        Err(e) => {
            println!(">> errror: {:?}", e);
            assert_eq!("hmm", format!("error: {:#?}", e))
        },
    }
}