


const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;


#[derive(Clone, Debug)]
pub struct GeoPosition {
    pub longitude: f64,
    pub latitude: f64,
}



pub fn geo_distance_between(long1: f64, lat1: f64, long2: f64, lat2: f64) -> f64 {

    let _dlon = (long2 - long1) * DEG_TO_RAD;
    let _dlat = (lat2 - lat1) * DEG_TO_RAD;
    let _lat1 = lat1 * DEG_TO_RAD;
    let _lat2 = lat2 * DEG_TO_RAD;

    let _a = f64::sin(_dlat/2.0) * f64::sin(_dlat/2.0) + f64::sin(_dlon/2.0) * f64::sin(_dlon/2.0) * f64::cos(_lat1) * f64::cos(_lat2);
    
    12742.0 * f64::atan2(f64::sqrt(_a), f64::sqrt(1.0 - _a))

}



impl GeoPosition {

    pub fn new(longitude: f64, latitude: f64) -> GeoPosition {

        GeoPosition {
            longitude,
            latitude
        }

    }

    pub fn distanceto_within_box(&self, longitude: f64, latitude: f64, offset: f64) -> f64 
    {        
        let _dlon = (self.longitude - longitude);
        let _dlat = (self.latitude - latitude);
        
        if _dlon.abs() > offset || _dlat.abs() > offset {
            return -1.0;
        }

        geo_distance_between(
            self.longitude, 
            self.latitude, 
            longitude, 
            latitude
        )
    }

    pub fn distanceto(&self, longitude: f64, latitude: f64) -> f64 
    {        
        geo_distance_between(
            self.longitude, 
            self.latitude, 
            longitude, 
            latitude
        )
    }

}