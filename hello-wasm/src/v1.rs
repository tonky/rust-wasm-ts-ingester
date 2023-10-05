use wasm_bindgen::prelude::*;
use rmp_serde::{Deserializer, Serializer};
use serde::{Serialize,Deserialize};
// use base64::engine;
// use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
use base64::{alphabet, engine, engine::general_purpose, Engine as _, DecodeError};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[wasm_bindgen]
pub struct NaiveDateTime {
    pub millisecond: u16
}


#[wasm_bindgen]
impl NaiveDateTime {
    #[wasm_bindgen(constructor)]
    pub fn new(ms: u16) -> NaiveDateTime {
        NaiveDateTime { millisecond: ms }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct MetricV1 {
    pub client_id: u32,
    pub auth_token: String,
    pub datetime: NaiveDateTime
}

#[wasm_bindgen]
impl MetricV1 {
    #[wasm_bindgen(constructor)]
    pub fn new(cid: u32, at: &str, dt: u16 ) -> MetricV1 {
        MetricV1 { client_id: cid, auth_token: at.into(), datetime: NaiveDateTime { millisecond: dt } }
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

        // const CUSTOM_ENGINE: engine::GeneralPurpose =
         // engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
        general_purpose::STANDARD.encode(&self.messagepacked())
        // CUSTOM_ENGINE.encode(&self.messagepacked())
        // String::from_utf8_lossy(&enc).to_string();
    }
}

#[wasm_bindgen]
pub struct V1 {
    url: String
}

#[wasm_bindgen]
impl V1 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> V1{
        V1 { url: "localhost:3000/v1/ingest".into() }
    }

    pub fn publish(&self, m: MetricV1) -> Result<String, JsError> {
        let msgp = m.messagepacked();

        // let mut buf = Vec::new();
        // m.serialize(&mut Serializer::new(&mut buf)).unwrap();

    // log(&format!("Rust log 2: {:?} - {:?}", m, &buf));

        let enc = general_purpose::STANDARD.encode(&msgp);
        println!("encoded b64: {:?}", enc);
        // String::from_utf8_lossy(&buf).to_string()
        // Ok(format!("Hello, b64 messagepack: {}!", enc))
        // Ok(enc)
        Ok(format!("posted {} on {}", enc, self.url))
    }
}