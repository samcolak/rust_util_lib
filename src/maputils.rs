
use serde_json::{Map, Value};

use std::collections::HashMap;
use std::collections::HashSet;


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
    match element.get(key) {
        None => default,
        Some(value) => {
            if let Some(number) = value.as_u64() {
                number
            } else {
                match value.as_str() {
                    Some(number) => number.parse::<u64>().unwrap_or(default),
                    None => default,
                }
            }
        }
    }
}

pub fn val_toi64(element: &Map<String, Value>, key: &str, default: i64) -> i64 {
    match element.get(key) {
        None => default,
        Some(value) => {
            if let Some(number) = value.as_i64() {
                number
            } else {
                match value.as_str() {
                    Some(number) => number.parse::<i64>().unwrap_or(default),
                    None => default,
                }
            }
        }
    }
}

pub fn val_tou32(element: &Map<String, Value>, key: &str, default: u32) -> u32 {    
    match element.get(key) {
        None => default,
        Some(value) => {
            if let Some(number) = value.as_u64() {
                number as u32
            } else {
                match value.as_str() {
                    Some(number) => number.parse::<u32>().unwrap_or(default),
                    None => default,
                }
            }
        }
    }
}

pub fn val_tobool(element: &Map<String, Value>, key: &str, default: bool) -> bool {
    match element.get(key) {
        None => default,
        Some(value) => {
            if let Some(boolean) = value.as_bool() {
                boolean
            } else if value.is_number() {
                value.as_i64() == Some(1)
            } else {
                match value.as_str() {
                    Some(boolean) => boolean.parse::<bool>().unwrap_or(default),
                    None => default,
                }
            }
        }
    }
}

pub fn val_tofloat(element: &Map<String, Value>, key: &str, default: f64) -> f64 {
    match element.get(key) {
        None => default,
        Some(value) => {
            if let Some(number) = value.as_f64() {
                number
            } else {
                match value.as_str() {
                    Some(number) => number.parse::<f64>().unwrap_or(default),
                    None => default,
                }
            }
        }
    }
}


pub fn from_hashmap<T: Clone>(mapin: HashMap<String, T>) -> Map<String, Value> where serde_json::Value: std::convert::From<T> {
    let mut out = Map::with_capacity(mapin.len());
    out.extend(mapin.into_iter().map(|(name, value)| (name, Value::from(value))));
    out
}


pub fn map_copy_exceptkeys(element: &Map<String, Value>, keys: Vec<&str>) -> Map<String, Value> {        
    let lookup: HashSet<&str> = keys.into_iter().collect();
    let mut out = Map::with_capacity(element.len());
    for (key, value) in element {
        if !lookup.contains(key.as_str()) {
            out.insert(key.clone(), value.clone());
        }
    }
    out
}


pub fn map_copy_withkeys(element: &Map<String, Value>, keys: Vec<&str>) -> Map<String, Value> {
    let lookup: HashSet<&str> = keys.into_iter().collect();
    let mut out = Map::with_capacity(element.len());
    for (key, value) in element {
        if lookup.contains(key.as_str()) {
            out.insert(key.clone(), value.clone());
        }
    }
    out
}