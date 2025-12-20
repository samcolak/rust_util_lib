

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


pub trait EnumType 
where Self: Sized + Copy + 'static + Into<u8> + From<u8> {
    
    fn default() -> Self;

    fn as_vec() -> Vec<&'static str>;

    fn from_str(s: &str) -> Self {
        if let Some(p) = Self::as_vec().iter().position(|n| *n == s) {
            (p as u8).into()
        } else {
            Self::default()
        }
    }

    fn to_str(&self) -> &str {
        Self::as_vec()[usize::from((*self).into())]
    }

    fn str_value(self) -> String {
        self.to_str().to_owned()
    }

}
  
