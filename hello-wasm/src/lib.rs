mod utils;

// use anyhow::Result;
use wasm_bindgen::prelude::*;
use serde_json::{Value, json};
use serde::{Serialize,Deserialize};
use thiserror::Error;
use rmp_serde::{Deserializer, Serializer};
use chrono::{DateTime, Utc, ParseError, ParseResult};
use base64::{engine::general_purpose, Engine as _, DecodeError};

/*
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
*/

#[derive(Error, Debug)]
pub enum MyError {
    #[error("{0}: {1}")]
    ChronoParseError(chrono::ParseError, String),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Metric {
    client_id: u32,
    auth_token: String,
    datetime: DateTime<chrono::Utc>
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    // alert("Hello, hello-wasm!");
    
    let m = Metric{client_id: 1, auth_token: "auth".into(), datetime: chrono::Utc::now()};

    println!(">> example json: {}", json!(m));

    let mut buf = Vec::new();
    m.serialize(&mut Serializer::new(&mut buf)).unwrap();

    println!("{:?} - {:?}", m, buf);

//*
    let enc = general_purpose::STANDARD.encode(&buf);
    println!("encoded b64: {:?}", enc);
//*/
    // String::from_utf8_lossy(&buf).to_string()
    format!("Hello, {}!", enc)
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

    let m = Metric{client_id, auth_token: auth_token.into(), datetime: chrono_dt.into()};

    // Ok(json!(m).to_string())
//*
    let mut buf = Vec::new();
    m.serialize(&mut Serializer::new(&mut buf)).unwrap();

    println!("{:?} - {:?}", m, buf);

    let enc = general_purpose::STANDARD.encode(&buf);
    println!("encoded b64: {:?}", enc);
    // String::from_utf8_lossy(&buf).to_string()
    // Ok(format!("Hello, b64 messagepack: {}!", enc))
    Ok(enc)
//*/

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