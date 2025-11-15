
use serde_json::{Map, Value};

use std::collections::HashMap;

use crate::serdeutils::ParseValue;


#[macro_export]
macro_rules! map_add {
    ($x:expr, $y:expr, $z:expr) => {
        $x.insert($y.to_string(), Value::from($z))
    };
}


pub fn val_tostr(element: &Map<String, Value>, key: &str, default: &str) -> String {
    if let Some(_k) = element.get(key) {
        if *_k == Value::Null {
            default.to_string()
        } else {
            _k.as_str().expect("").to_string()
        }
    } else {
        default.to_string()
    }
} 

pub fn val_tou64(element: &Map<String, Value>, key: &str, default: u64) -> u64 {
    u64::from(ParseValue(&serde_json::Value::Object(element.clone()), key, default))
}

pub fn val_toi64(element: &Map<String, Value>, key: &str, default: i64) -> i64 {
    i64::from(ParseValue(&serde_json::Value::Object(element.clone()), key, default))
}

pub fn val_tou32(element: &Map<String, Value>, key: &str, default: u32) -> u32 {    
    u32::from(ParseValue(&serde_json::Value::Object(element.clone()), key, default))
}

pub fn val_tobool(element: &Map<String, Value>, key: &str, default: bool) -> bool {
    bool::from(ParseValue(&serde_json::Value::Object(element.clone()), key, default))
}

pub fn val_tofloat(element: &Map<String, Value>, key: &str, default: f64) -> f64 {
    f64::from(ParseValue(&serde_json::Value::Object(element.clone()), key, default))
}


pub fn from_hashmap<T: Clone>(mapin: HashMap<String, T>) -> Map<String, Value> where serde_json::Value: std::convert::From<T> {
    let mut _out = Map::new();
    _out.extend(mapin.iter().map(|(_n, _v)| (_n.clone(), Value::from(_v.clone()))));
    _out
}


pub fn map_copy_exceptkeys(element: &Map<String, Value>, keys: Vec<&str>) -> Map<String, Value> {        
    let mut _out = Map::new();
    for (_key, _value) in element {
        if !keys.contains(&_key.as_str()) {
            _out.insert(_key.to_string(), _value.clone());
        }
    }
    _out
}


pub fn map_copy_withkeys(element: &Map<String, Value>, keys: Vec<&str>) -> Map<String, Value> {
    let mut _out = Map::new();
    for (_key, _value) in element {
        if keys.contains(&_key.as_str()) {
            _out.insert(_key.to_string(), _value.clone());
        }
    }
    _out
}