

use serde_json::{Value};
use serde::{Serialize};


pub trait Jsonify <T: Serialize + Clone> {
    fn to_value(&self) -> Value;
    fn to_string(&self) -> String;
    fn from_string(stringin: &str) -> Result<T, serde_json::Error>;
    fn from_value(valuein: &Value) -> Result<T, serde_json::Error>;
}


pub trait ByteTransform {
    fn to_bytes(&self) -> &[u8];
    fn from_bytes(bytesin: &[u8]) -> String;   
}


pub trait Base64Transform <T: Serialize + Clone> {
    fn to_base64(&self) -> String;
    fn from_base64(stringin: &str) -> Option<T>;
}


pub trait EnumType {
    fn enums() -> Vec<&'static str>;
}