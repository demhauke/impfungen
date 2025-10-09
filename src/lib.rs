use serde_json;
use serde::{ Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Impfung{
    name: String,
    date: String,
    expiration_years: usize,
}

// impl Serialize for Impfung {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
        
//     }
// }

impl Impfung {
    fn get_time_left(&self) -> usize {
        10
    }

    pub fn new(name: String, date: String, expiration_years: usize) -> Impfung {
        Impfung { name: name, date: date, expiration_years: expiration_years }
    }
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).expect("String failed")
    }
    pub fn from_str(str: &str) -> Result<Impfung, ()> {
        match serde_json::from_str(str) {
            Ok(value) => value,
            Err(err) => Err(()),
        }
        
        
        //serde_json::from_str(str)
    }
}