
use std::collections::HashMap;
use std::time::{SystemTime};

use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

use chrono::{DateTime};
use totp_rs::{Algorithm, TOTP};

use rand::{distributions::Alphanumeric, Rng};

use uuid::Uuid;

use serde_json::{Value, Map};


#[macro_export]
macro_rules! str {
    () => {
        String::new()
    };
    ($x:expr $(,)?) => {
        ToString::to_string(&$x)
    };
}


// base64 encoding...

pub fn b64_encode_bytes(bytesin: &[u8]) -> String {
    general_purpose::STANDARD_NO_PAD.encode(bytesin)
}


pub fn b64_encode_withengine(bytesin: &[u8], engine: general_purpose::GeneralPurpose) -> String {
    engine.encode(bytesin)
}


pub fn b64_encode(stringin: &str) -> String {
    let stringfull = stringin.to_string();
    let orig = stringfull.as_bytes();
    b64_encode_bytes(orig)
}


pub fn b64_decode_withconfig(stringin: &str, config: general_purpose::GeneralPurposeConfig) -> Vec<u8> {
    let baseengine: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::URL_SAFE, config);
    match baseengine.decode(stringin) {
        Ok(_n) => _n,
        _ => {
            println!("Decoding error...");
            Vec::new()
        },
    }
}



pub fn b64_decode_withengine(stringin: &str, engine: general_purpose::GeneralPurpose) -> Vec<u8> {
    match engine.decode(stringin) {
        Ok(_c) => _c,
        Err(_e) => {
            Vec::new()
        }
    }
}


pub fn b64_decode(stringin: &str) -> Vec<u8> {
    let mut _result = b64_decode_withengine(stringin, general_purpose::STANDARD);
    if _result.is_empty() {
        _result = b64_decode_withengine(stringin, general_purpose::STANDARD_NO_PAD);
        if _result.is_empty() {
            _result = b64_decode_withengine(stringin, general_purpose::URL_SAFE);
            if _result.is_empty() {
                _result = b64_decode_withengine(stringin, general_purpose::URL_SAFE_NO_PAD);
            }
        }    
    }
    _result
}


pub fn b64_decode_tostring(stringin: &str) -> String {
    let bytes = b64_decode(stringin);
    String::from_utf8_lossy(&bytes).to_string()
}


// hashmap utilites...

pub fn map_to_value(map: &HashMap<String, String>) -> serde_json::Value {
    let mut _out = Map::new();
    for _key in map.keys() {
        _out.insert(_key.to_string(), Value::from(map.get(_key).expect("").to_string()));
    }
    Value::from(_out)
}

pub fn map_strvalue(map: &HashMap<String, String>, key: &str, default: &str) -> String {
    match map.get(key) {
        Some(_c) => _c.to_string(),
        _ => default.to_string()
    }
}

pub fn get_topt_token(secret: &str, epoch: i64) -> String {
    let totp = TOTP::new(
        Algorithm::SHA1,
        6, // a string of 6 chars...
        1,
        30, // period (30 seconds)
        secret.to_string().as_bytes().to_vec(),
        None,
        "".to_string()
    ).unwrap();

    totp.generate(epoch.try_into().unwrap())
}

pub fn random_alpha(count: i64) -> String {

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(count.try_into().unwrap()).collect()

}


// Standard utils we all know and love...

pub fn epoch() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(_n) => _n.as_secs().try_into().unwrap(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn epoch_to_utcdate(epochin: i64, format: &str) -> String {    
    let _datetime= DateTime::from_timestamp(epochin, 0).unwrap();
    _datetime.format(format).to_string()
}


pub fn epoch_real() -> f64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(_n) => _n.as_secs_f64(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}


pub fn unique_id() -> String {    
    Uuid::now_v7().to_string().replace("-", "")    
}


pub fn hash(stringin: &str) -> String {
    md5(stringin.as_bytes())
}


pub fn md5(bytes : &[u8]) -> String {
    let _digest = md5::compute(bytes);
    format!("{:x}", _digest)
}


#[cfg(test)]
mod test {

    use super::b64_encode;
    use super::b64_decode_tostring;
    
    #[test]
    fn base64tests() {

        // inject signing items into the list...

        let stringtotest = "Hello World!".to_string();

        let encoded = b64_encode(&stringtotest);
        let decoded = b64_decode_tostring(&encoded);

        assert_eq!(stringtotest, decoded);

    }

}