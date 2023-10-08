mod utils;
pub mod v1;
pub mod v2;

// use anyhow::Result;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use serde_wasm_bindgen;
use web_sys::{console, Request, RequestInit, RequestMode, Response, Url};
use serde_json::{Value, json};
use serde::{Serialize,Deserialize};
use thiserror::Error;
use rmp_serde::{Deserializer, Serializer};
use chrono::{DateTime, Utc, ParseError, ParseResult};
use base64::{engine::general_purpose, Engine as _, DecodeError};

// #[wasm_bindgen]
pub enum Metric {
    V1(v1::MetricV1)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgResponse {
    pub status: String
}

/*
#[wasm_bindgen]
pub fn new_v1() -> v1::V1 {
    v1::V1::new()
}
 */

#[derive(Error, Debug)]
pub enum MyError {
    #[error("{0}: {1}")]
    ChronoParseError(chrono::ParseError, String),
}

/*
#[derive(Debug, PartialEq, Deserialize, Serialize)]
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
*/

/*
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Metric {
    pub client_id: u32,
    pub auth_token: String,
    datetime: DateTime<chrono::Utc>
}

#[wasm_bindgen]
impl Metric {
    #[wasm_bindgen(constructor)]
    pub fn new(cid: u32, at: &str ) -> Metric {
        Metric { client_id: cid, auth_token: at.into(), datetime: chrono::Utc::now() }
    }

    pub fn mp(&self) -> String {
        format!("Id: {}", self.client_id)
    }
}
 */

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello again, {}!", name)
}

#[wasm_bindgen]
pub fn message(client_id: u32, auth_token: &str, datetime: &str) -> Result<String, JsError> {
    // alert("Hello, hello-wasm!");
    /*
    let date_str = "2020-04-12T22:10:57+02:00";
// convert the string into DateTime<FixedOffset>
let datetime = DateTime::parse_from_rfc3339(date_str).unwrap();
// convert the string into DateTime<Utc> or other timezone
let datetime_utc = datetime.with_timezone(&Utc);
     */

   // let chrono_dt = DateTime::parse_from_rfc3339(datetime)?;
   //*
    let chrono_dt = match DateTime::parse_from_rfc3339(datetime) {
        Ok(chrono_fixed) => chrono_fixed,
        // Err(e) => return Err(JsError::new(format!("parse error on '{}': {}", datetime, e)))
        // Err(e) => return std::fmt::Error(format!("parse error on '{}': {}", datetime, e))
        Err(e) => return Err(MyError::ChronoParseError(e, datetime.into()).into())
        // Error(e) => { Err(JsError::new("message"))}
    };

    let m = common::MetricV1{client_id, auth_token: auth_token.into(), datetime: chrono_dt.into()};

    // Ok(json!(m).to_string())
//*
    let mut buf = Vec::new();
    m.serialize(&mut Serializer::new(&mut buf)).unwrap();

    console::log_1(&"Hello using web-sys".into());
    // log(&format!("Rust log 2: {:?} - {:?}", m, &buf));

    let enc = general_purpose::STANDARD.encode(&buf);
    println!("encoded b64: {:?}", enc);
    // String::from_utf8_lossy(&buf).to_string()
    // Ok(format!("Hello, b64 messagepack: {}!", enc))
    Ok(enc)
//*/

}

#[wasm_bindgen]
pub async fn post_metric(url: Url, body: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!(">> calling post on: {}", url.href()).into());

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    /*
    let metric_json = match metric {
        v1::MetricV1(m) => m.msgpk_b64(),
        _ => String::from("{}")
    };
 */
    opts.body(Some(&JsValue::from_str(&body)));

    // let url = format!("http://localhost:3000{}", path);

    let request = Request::new_with_str_and_init(&url.href(), &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Use serde to parse the JSON into a struct.
    console::log_2(&">> got json: ".into(), &json);
    let resp: common::MetricsResponse = serde_wasm_bindgen::from_value(json)?;

    // Send the `Branch` struct back to JS as an `Object`.
    // Ok(JsValue::from_serde(&resp).unwrap())
    Ok(serde_wasm_bindgen::to_value(&resp)?)
    // Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_check() {
        let greeting = greet("Joe");
        assert_eq!(greeting, String::from("Hello, Joe!"));
    }
}