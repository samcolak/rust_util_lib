
#![allow(clippy::should_implement_trait)]

use std::collections::HashMap;

use warp::http::HeaderMap;

use flate2::{write::GzEncoder, Compression};
use brotli::CompressorWriter;
use std::io::Write;

use serde_json::{Map, Value};
use serde::{Deserialize, Serialize};

use regex::Regex;

use crate::str;

use crate::gentraits::EnumType;
// use crate::helpers::traits::Jsonify;
use crate::serdeutils::{fetch_string};

// http return codes...

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
#[repr(u16)]
pub enum HttpResponseCodes {
	Continue = 100,
	SwitchingProtocol,
    Processing,
	EarlyHints,
	OK = 200,
    Created,
	Accepted,
	NonAuthoritativeInformation,
	NoContent,
	ResetContent,
	PartialContent,
	MultiStatus,
	AlreadyReported,
	IMUsed = 226,
	MultipleChoice = 300,
	MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
	TemporaryRedirect = 307,
    PermanentRedirect,
    BadRequest = 400,
	Unauthorized = 401,
	PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    Imateapot,
    MisdirectedRequest = 421,
    UnprocessedEntity,
    Locked,
    FailedDependancy,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons,
    InternalServerError = 500,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended = 510,
    NetworkAuthenticationRequired
}

impl From<HttpResponseCodes> for u16 {
    fn from(val: HttpResponseCodes) -> Self {
        val as u16
    }
}


pub fn headermap_tohashmap(
    
    mapin: &HeaderMap

) -> HashMap<String, String> {

    let mut _out = HashMap::new();

    for (key, value) in mapin {
        if *key != "cookie" { // filter cookies..
            _out.insert(key.to_string(), value.to_str().unwrap().to_string());
        }
    }
    
    _out

}


pub fn is_ajax(
    headers: &HeaderMap
) -> bool {
    
    let mut _ajax = false;
    
    if headers.contains_key("http_x_requested_with") {
        _ajax = headers["http_x_requested_with"] == "XMLHttpRequest";
    } else if headers.contains_key("x-requested-with") {
        _ajax = headers["x-requested-with"] == "XMLHttpRequest";
    }

    _ajax

}


pub fn get_language(
    
    codesin: String

) -> String {

    let mut _selected: String = "en".to_string();
    let mut _preferences: Vec<String> = Vec::new();

    if !codesin.is_empty() {

        let parser = Regex::new(r"([a-z]{2}[^0-9,.;\-= ]?)").unwrap();        
        let langs_found: Vec<_> = parser.find_iter(&codesin).map(|mat| mat.as_str()).collect();

        if !langs_found.is_empty() {
            for langraw in langs_found {
                let lang_to_add = langraw.to_lowercase().to_string();
                if !_preferences.contains(&lang_to_add) {
                    _preferences.push(lang_to_add);
                }
            }
        }

        if !_preferences.is_empty() {
            _selected = _preferences[0].clone();
        }

    }

    _selected

}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub ipaddress: String,
    pub countrycode: String,
    pub city: String,
    pub longitude: f64,
    pub latitude: f64,
}


pub fn lookup_mime(extension: &str) -> &str {
    match extension {
        "css" => "text/css",
        "gif" => "image/gif",
        "ico" => "image/x-icon",
        "jpeg" => "image/jpeg",
        "jpg" => "image/jpg",
        "js" => "application/javascript",
        "text" => "text/plain",
        "txt" => "text/plain",
        "png" => "image/png",
        "pl" => "text/plain",
        "ttf" => "application/x-font-truetype",
        "otf" => "application/x-font-opentype",
        "woff" => "application/font-woff",
        "eot" => "application/vnd.ms-fontobject",
        "svg" => "image/svg+xml",
        "woff2" => "application/font-woff2",
        "ics" => "text/calendar",
        "mp4" => "application/mp4",
        "vcf" => "text/vcard",
        "pdf" => "application/pdf",
        "csv" => "text/csv",
        "html" => "text/html",
        "raw" => "text/plain",
        "json" => "application/json",
        "xml" => "application/xml",
        "xmls" => "application/xml",
        "pem" => "application/x-x509-ca-cert",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "ppt" => "application/vnd.ms-powerpoint",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "doc" => "application/msword",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xls" => "application/vnd.ms-excel",
        _ => "text/html"
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum HttpMethod {
    Undefined = 0,
    Gateway = 1,
	Get = 2,
    Post = 3,
	Put = 4,
    Delete = 5,
    User = 6,
    Head = 7,
    Sock = 8
}


impl Into<u8> for HttpMethod {
    fn into(self) -> u8 {
        self as u8
    }
}


impl From<u8> for HttpMethod {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Gateway,
            2 => Self::Get,
            3 => Self::Post,
            4 => Self::Put,
            5 => Self::Delete,
            6 => Self::User,
            7 => Self::Head,
            8 => Self::Sock,
            _ => Self::Undefined
        }
    }
}


impl EnumType for HttpMethod {

    fn as_vec() -> Vec<&'static str> {
        ["undefined", "gateway", "get", "post", "put", "delete", "user", "head", "sock"].to_vec()
    }

    fn default() -> Self {
        Self::Undefined
    }

}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebRepeaterDef {
    pub display_as: Option<String>,
    pub liclass: Option<String>,
    pub class: Option<String>,
    pub domain: Option<String>,
    pub action: Option<String>,
    pub args: Option<Map<String, Value>>,
    pub data: Map<String, Value>  
}


impl WebRepeaterDef {

    pub fn new(

        display_as: &str,
        liclass: &str,
        class: &str,
        domain: &str,
        action: &str,
        args: Option<Map<String, Value>>

    ) -> Self {

        WebRepeaterDef {
            display_as: Some(str!(display_as)),
            liclass: Some(str!(liclass)),
            class: Some(str!(class)),
            domain: Some(str!(domain)),
            action: Some(str!(action)),
            args,
            data: Map::new()
        }

    }

    pub fn to_map(&self) -> serde_json::Value {        
        serde_json::to_value(self).unwrap()
    }

    pub fn from_map(map: serde_json::Value) -> Self {        

        let argsout = match fetch_string(&map, "args", "") {
            Some(_a) => _a,
            _ => str!("{}")
        };

        let dataout = match fetch_string(&map, "data", "") {
            Some(_d) => _d,
            _ => str!("{}")
        };

        WebRepeaterDef {
            display_as: fetch_string(&map, "display_as", ""),
            liclass: fetch_string(&map, "liclass", ""),
            class: fetch_string(&map, "class", ""),
            domain: fetch_string(&map, "domain", ""),
            action: fetch_string(&map, "action", ""),
            args: serde_json::from_str(&argsout).unwrap(),
            data: serde_json::from_str(&dataout).unwrap(),
        }
        
    }

}



pub fn cleanup_html(stringin: &str) -> String {
    
    let mut _toclean = stringin.to_string()
        .replace("<br>", "")
        .replace("<p>", "")
        .replace("</p>", "");

    let _tags = ["a", "img", "ul", "iframe", "br"].to_vec();

    for _tag in _tags {

        let _synch = format!(r"(?:<({_tag})*?>.+?<\/({_tag})>|<({_tag}).+?>.+?<\/({_tag})>|<(?:!|\/?\*).*?\/?>)|<({_tag})><\/({_tag})>/gm");
        let _regx = Regex::new(&_synch).unwrap();
        let _copy = _toclean.clone();

        let _partsfound: Vec<_> = _regx.find_iter(&_copy).map(|mat| mat.as_str()).collect();

        for _part in _partsfound {
            _toclean = _toclean.replace(_part, "");
        }      

    }
    
    _toclean

}



#[derive(PartialEq, Debug, Serialize, Eq, Clone, Copy)]
#[repr(u8)]
pub enum HttpEncoding {
    Brotli = 0,
    Gzip = 1,
    Deflate = 2,  
    Identity = 3,
}


impl From<u8> for HttpEncoding {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Brotli,
            1 => Self::Gzip,
            2 => Self::Deflate,
            _ => Self::Identity
        }
    }
}

impl Into<u8> for HttpEncoding {
    fn into(self) -> u8 {
        self as u8
    }
}


const SUPPORTED_ENCODINGS: &[&str] = &["br", "gzip", "deflate", "identity"];


impl EnumType for HttpEncoding {
    
    fn as_vec() -> Vec<&'static str> {
        SUPPORTED_ENCODINGS.to_vec()
    }

    fn default() -> Self {
        Self::Identity
    }

}


impl From<&str> for HttpEncoding {
    
    fn from(value: &str) -> Self {
        match value {
            "br" => HttpEncoding::Brotli,
            "gzip" => HttpEncoding::Gzip,
            "deflate" => HttpEncoding::Deflate,
            _ => HttpEncoding::Identity
        }
    }

}


impl HttpEncoding {

    pub fn compress(&self, body: &[u8]) -> Vec<u8> {

        match self {

            Self::Gzip => {
                let mut _encoder = GzEncoder::new(Vec::new(), Compression::default());
                _encoder.write_all(body).unwrap();
                _encoder.finish().unwrap()
            },

            Self::Deflate => {
                let mut _encoder = flate2::write::DeflateEncoder::new(Vec::new(), Compression::default());
                _encoder.write_all(body).unwrap();
                _encoder.finish().unwrap()
            },

            Self::Brotli => {
                let mut _output = Vec::new();
                {
                    let mut _writer = CompressorWriter::new(&mut _output, 4096, 11, 22);
                    _writer.write_all(body).unwrap();
                }
                _output.to_vec()
            }

            _ => body.to_vec()
            
        }

    }
    
}


pub fn determine_encoding(header: Option<String>) -> HttpEncoding {

    match header {

        Some(value) => {
            let mut _encodings: Vec<(String, f32)> = value
                .split(',')
                .filter_map(|_enc| {
                    let mut _parts = _enc.trim().split(';');
                    let _encoding = _parts.next()?.trim().to_ascii_lowercase();
                    let _q = _parts
                        .find_map(|_part| {
                            if _part.trim().starts_with("q=") {
                                _part.trim()[2..].parse::<f32>().ok()
                            } else {
                                None
                            }
                        })
                        .unwrap_or(1.0);
                    Some((_encoding, _q))
                })
                .filter(|(_, _q)| *_q > 0.0)
                .collect();

            // Sort by quality descending
            _encodings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            // Return the first matching supported encoding
            for (_encoding, _) in _encodings {
                if SUPPORTED_ENCODINGS.contains(&_encoding.as_str()) {
                    return _encoding.as_str().into();
                }
            }

            // Fallback
            HttpEncoding::Identity
        }

        None => HttpEncoding::Identity,
    
    }

}


