
use serde_json::Value;


pub struct ParseValue<'a, T>(pub &'a Value, pub &'a str, pub T);

impl <'a> From<ParseValue<'a, u64>> for u64 {

    fn from(src: ParseValue<u64>) -> u64 {
        match src.0.get(src.1) {
            None => src.2,
            Some(e) => {
                if let Some(number) = e.as_u64() {
                    number
                } else {            
                    match e.as_str() {
                        Some(num) => match num.parse::<u64>() {
                            Ok(parsed) => parsed,
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
                if let Some(number) = e.as_u64() {
                    number as u32
                } else {            
                    match e.as_str() {
                        Some(num) => match num.parse::<u32>() {
                            Ok(parsed) => parsed,
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
                if let Some(number) = e.as_i64() {
                    number
                } else {            
                    match e.as_str() {
                        Some(num) => match num.parse::<i64>() {
                            Ok(parsed) => parsed,
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
                if let Some(number) = e.as_f64() {
                    number
                } else {            
                    match e.as_str() {
                        Some(num) => match num.parse::<f64>() {
                            Ok(parsed) => parsed,
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
                if let Some(boolean) = e.as_bool() {
                    boolean
                } else if e.is_number() {                
                    e.as_i64() == Some(1)
                } else {
                    match e.as_str() {
                        Some(num) => match num.parse::<bool>() {
                            Ok(parsed) => parsed,
                            Err(_) => src.2,
                        },
                        _ => src.2
                    }      
                }
            }
        }
    }

}


fn parse_index_segment(segment: &str) -> Option<(&str, usize)> {
    let open_index = segment.find('[')?;
    let close_index = segment[open_index + 1..].find(']')? + open_index + 1;

    if close_index <= open_index + 1 {
        return None;
    }

    let index_text = &segment[open_index + 1..close_index];
    let index = index_text.parse::<usize>().ok()?;
    Some((&segment[..open_index], index))
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
    let mut change = String::new();

    let (vartolookup, newpath) = match path.split_once('.') {
        Some((head, tail)) => (head, tail),
        None => (path, ""),
    };

    if !vartolookup.is_empty() {
        if let Some(string_value) = vars.as_str() {
            change = string_value.to_string();
            return change;
        }

        if vars.is_array() || vars.is_object() {
            if let Some((varname, index)) = parse_index_segment(vartolookup) {
                if varname.is_empty() {
                    if let Some(values) = vars.as_array() {
                        if let Some(value_to_recurse) = values.get(index) {
                            return recurse_value(newpath, value_to_recurse);
                        }
                    }
                    return "Undefined".to_string();
                }

                if let Some(object_to_recurse) = vars.as_object() {
                    if let Some(values) = object_to_recurse.get(varname).and_then(|v| v.as_array()) {
                        if let Some(value_to_recurse) = values.get(index) {
                            return recurse_value(newpath, value_to_recurse);
                        }
                    }
                }
                return "Undefined".to_string();
            }

            if let Some(object_to_recurse) = vars.as_object() {
                if let Some(value_to_recurse) = object_to_recurse.get(vartolookup) {
                    change = recurse_value(newpath, value_to_recurse);
                }
                return change;
            }
        }
    }

    if change.is_empty() {
        if vars.is_null() {
        } else if let Some(string_value) = vars.as_str() {
            change = string_value.to_string();
        } else {
            change = serde_json::to_string(vars).expect("No variable found with this name");
        }
    }

    log::debug!("Returning value {:?}", &change);

    change

}