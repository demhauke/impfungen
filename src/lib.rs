use serde_json;
use serde_json::json;
use serde::{Deserialize, Serialize};
use chrono::{Datelike, Duration, NaiveDate, Utc};

#[derive(Serialize, Deserialize)]
pub struct Impfung{
    pub name: String,
    pub date: NaiveDate,
    pub expiration_years: i32,
}

enum TimeLeft {
    NoTimeLeft,
    TimeLeft(NaiveDate)
}


impl Impfung {

    // pub fn get_time_left(&self) -> Option<NaiveDate> {
    //     NaiveDate::from_ymd_opt(self.date.year() + self.expiration_years, self.date.month0(), self.date.day0())
    // }

    // // Option weil impfung auch abgelaufen sein kann
    // pub fn get_time_left(&self, current_year: NaiveDate) -> TimeLeft {
    //     (self.date - current_year)
    // }

    pub fn new(name: String, date: NaiveDate, expiration_years: i32) -> Impfung {
        // let impfung__ = Impfung { name: name.clone(), date: date.clone(), expiration_years: expiration_years };
        // let impfungsstext = impfung__.to_string();
        // println!("struct as string: {}", impfungsstext);
        // println!("struct from str: {}", Impfung::from_str(&impfungsstext).unwrap().name);
        // println!("struct from str: {}", Impfung::from_str(&impfungsstext).unwrap().date);
        // println!("struct from str: {}", Impfung::from_str(&impfungsstext).unwrap().expiration_years);
        // println!("struct as string from str from struct: {}", Impfung::to_string(&Impfung::from_str(&impfungsstext).unwrap()));
        Impfung { name: name, date: date, expiration_years: expiration_years }
    }

    fn to_string(&self) -> String{
        json!(
            {
                "name": self.name,
                "date": self.date,
                "expiration_years": self.expiration_years,
            }
        ).to_string()
    }

    fn from_str(str: &str) -> Option<Impfung>{
        serde_json::from_str(str).ok()
    }



    // pub fn to_string(&self) -> String {
    //     serde_json::to_string(self).expect("String failed")
    // }
    // pub fn from_str(str: &str) -> Result<Impfung, ()> {
    //     match serde_json::from_str(str) {
    //         Ok(value) => value,
    //         Err(err) => Err(()),
    //     }
        
        
    //     //serde_json::from_str(str)
    // }
}