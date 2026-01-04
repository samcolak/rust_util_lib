
use std::time::{SystemTime};
use chrono::{DateTime};


#[macro_export]
macro_rules! epoch {
    () => {
        $crate::timeutils::epoch()
    };
}


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


// fast function to enumerate the week number since epoch - 1jan1970 (epoch 0) was a thursday...
pub fn weeknum_utc(epochin: i64) -> i64 {
    if epochin < 0 {
        return 0;
    }   
    1 + ((epochin + 345600) / 604800)
}