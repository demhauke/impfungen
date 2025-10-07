pub struct Impfung{
    name: String,
    date: String,
    expiration_years: usize,
}

impl Impfung {
    fn get_time_left(&self) -> usize {
        10
    }

    pub fn new(name: String, date: String, expiration_years: usize) -> Impfung {
        Impfung { name: name, date: date, expiration_years: expiration_years }
    }
}