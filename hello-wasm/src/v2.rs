use wasm_bindgen::prelude::*;
use rmp_serde::{Deserializer, Serializer};
use serde::{Serialize,Deserialize};
// use base64::engine;
// use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
use base64::{alphabet, engine, engine::general_purpose, Engine as _, DecodeError};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Url};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[wasm_bindgen]
pub struct DateTime {
    pub year: u16,
    pub millisecond: u16
}

#[wasm_bindgen]
impl DateTime {
    #[wasm_bindgen(constructor)]
    pub fn new(year: u16, ms: u16) -> DateTime {
        DateTime { year: year, millisecond: ms }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Metric {
    pub client_id: u32,
    pub auth_token: String,
    pub datetime: DateTime
}

#[wasm_bindgen]
impl Metric {
    #[wasm_bindgen(constructor)]
    pub fn new(cid: u32, at: &str, dt: DateTime ) -> Metric {
        Metric { client_id: cid, auth_token: at.into(), datetime: dt }
    }

    pub fn mp(&self) -> String {
        format!("Id: {}", self.client_id)
    }

    pub fn messagepacked(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        // let m = Metric{client_id: 1, auth_token: "auth".into(), datetime: chrono::Utc::now()};
        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        buf
    }

    pub fn msgpk_b64(&self) -> String {
        general_purpose::STANDARD.encode(&self.messagepacked())
    }
}

#[wasm_bindgen]
pub struct V2 {
    url: Url
}

#[wasm_bindgen]
impl V2 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> V2{
        V2 { url: Url::new("http://localhost:3000/v2/ingest").unwrap() }
    }

    pub async fn publish(&self, m: Metric) -> Result<JsValue, JsValue> {
        let body = m.msgpk_b64();

        super::post_metric(self.url.clone(), body).await
    }
}