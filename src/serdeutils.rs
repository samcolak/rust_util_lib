
use serde_json::Value;
use substring::Substring;
use regex::Regex;


pub struct ParseValue<'a, T>(pub &'a Value, pub &'a str, pub T);

impl <'a> From<ParseValue<'a, u64>> for u64 {

    fn from(src: ParseValue<u64>) -> u64 {
        match src.0.get(src.1) {
            None => src.2,
            Some(e) => {
                if e.is_u64() {            
                    e.as_u64().unwrap()
                } else {            
                    match e.as_str() {
                        Some(num) => match num.parse::<u64>() {
                            Ok(num) => num,
                            Err(_) => src.2,
                        },
                        _ => src.2,
                    }                        
                }
            }
        }
    }

}


impl <'a> From<ParseValue<'a, u32>> for u32 {

    fn from(src: ParseValue<u32>) -> u32 {
        match src.0.get(src.1) {
            None => src.2,
            Some(e) => {
                if e.is_u64() {            
                    e.as_u64().unwrap() as u32
                } else {            
                    match e.as_str() {
                        Some(num) => match num.parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => src.2,
                        },
                        _ => src.2,
                    }                        
                }
            }
        }
    }

}


impl <'a> From<ParseValue<'a, i64>> for i64 {

    fn from(src: ParseValue<i64>) -> i64 {
        match src.0.get(src.1) {
            None => src.2,
            Some(e) => {
                if e.is_i64() {            
                    e.as_i64().unwrap()
                } else {            
                    match e.as_str() {
                        Some(num) => match num.parse::<i64>() {
                            Ok(num) => num,
                            Err(_) => src.2,
                        },
                        _ => src.2,
                    }                        
                }
            }
        }
    }

}


impl <'a> From<ParseValue<'a, f64>> for f64 {

    fn from(src: ParseValue<f64>) -> f64 {
        match src.0.get(src.1) {
            None => src.2,
            Some(e) => {
                if e.is_f64() {            
                    e.as_f64().unwrap()
                } else {            
                    match e.as_str() {
                        Some(num) => match num.parse::<f64>() {
                            Ok(num) => num,
                            Err(_) => src.2,
                        },
                        _ => src.2,
                    }                        
                }
            }
        }
    }

}


impl <'a> From<ParseValue<'a, bool>> for bool {

    fn from(src: ParseValue<bool>) -> bool {
        match src.0.get(src.1) {
            None => src.2,
            Some(e) => {
                if e.is_boolean() {
                    e.as_bool().unwrap()              
                } else if e.is_number() {                
                    let numrep = e.as_number().unwrap();
                    numrep.as_i64() == Some(1)
                } else {
                    match e.as_str() {
                        Some(num) => match num.parse::<bool>() {
                            Ok(num) => num,
                            Err(_) => src.2,
                        },
                        _ => src.2
                    }      
                }
            }
        }
    }

}



pub fn fetch_string(element: &serde_json::Value, key: &str, default: &str) -> Option<String> {

    let strout = match element.get(key) {
        Some(e) => {
            if e.is_string() {
                e.as_str().expect("").to_string()
            } else {
                default.to_string()
            }
        },
        None => default.to_string()
    };

    if !strout.is_empty() {
        Some(strout)
    } else {
        None
    }

}   


pub fn parse_u32(element: &serde_json::Value, key: &'static str, default: u32) -> u32 {
    u32::from(ParseValue(element, key, default))
}


pub fn parse_u64(element: &serde_json::Value, key: &'static str, default: u64) -> u64 {
    u64::from(ParseValue(element, key, default))
}


pub fn parse_i64(element: &serde_json::Value, key: &'static str, default: i64) -> i64 {
    i64::from(ParseValue(element, key, default))
}


pub fn parse_bool(element: &serde_json::Value, key: &'static str, default: bool) -> bool {
    bool::from(ParseValue(element, key, default))
}


pub fn parse_f64(element: &serde_json::Value, key: &'static str, default: f64) -> f64 {
    f64::from(ParseValue(element, key, default))
}


pub fn recurse_value(
    
    path: &str,
    vars: &Value

) -> String {

    let mut _change:String = "".to_string();
    let _parts = path.split(".").collect::<Vec<&str>>();
        
    if !_parts.is_empty() {

        let mut _vartolookup: String = _parts[0].to_string();
        let mut _newpath: String = "".to_string();
        
        if _parts.len() > 1 {
            _newpath = _parts[1..].join(".").to_string();
        }

        if !_vartolookup.is_empty() { // not empty...

            if vars.is_string() {

                _change = vars.as_str().expect("No String").to_string();
            
            } else if vars.is_array() || vars.is_object() {

                let _parser = Regex::new(r"(\[)\d(\])?").unwrap();
                let _pieces: Vec<_> = _parser.find_iter(&_vartolookup).collect();
                let mut _index: usize = 0;

                // we have an indexer...   
                if !_pieces.is_empty() {

                    let _piece = _pieces[0];
                    let _varname = _vartolookup.substring(0, _piece.start());                    
                    _index = _vartolookup.substring(_piece.start()+1, _piece.end()-1).parse::<i64>().unwrap().try_into().unwrap();    

                    if _varname.is_empty() {
                        
                        let _vectorecurse = vars.as_array().into_iter().collect::<Vec<_>>();
                        if _index < _vectorecurse.len() {
                            let valuetorecurse = _vectorecurse[_index][0].clone();
                            return recurse_value(&_newpath, &valuetorecurse);
                        }
                        return "Undefined".to_string();

                    } else {

                        let _objecttorecurse = vars.as_object().expect("No Object"); 
                        if _objecttorecurse.contains_key(_varname) {
                            let _vectorecurse = _objecttorecurse.get(_varname).expect("No item found").as_array().into_iter().collect::<Vec<_>>();
                            if _index < _vectorecurse.len() {
                                let _valuetorecurse = _vectorecurse[_index][0].clone();
                                return recurse_value(&_newpath, &_valuetorecurse);
                            }
                        }
                        return "Undefined".to_string();

                    }

                } else {

                    let objecttorecurse = vars.as_object().expect("No Object"); 

                    if objecttorecurse.contains_key(&_vartolookup) {
                        let valuetorecurse: Value = objecttorecurse.get(&_vartolookup).expect("No item found").clone();
                        _change = recurse_value(&_newpath, &valuetorecurse)
                    } else {
                        _change = "".to_string();
                    }

                }

            }

            return _change;
            
        }

    }

    if _change.is_empty() {

        if vars.is_null () {
            // dont return a value - its null
        } else if vars.is_string() {
            _change = vars.as_str().expect("No String").to_string();
        } else {
            _change = serde_json::to_string(&vars).expect("No variable found with this name");
        }

    }

    log::debug!("Returning value {:?}", &_change);

    _change

}