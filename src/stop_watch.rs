

pub struct StopWatch{
    pub time_delta_root: Option<f64>,
}

impl StopWatch{
    pub fn get_time_since_last_called(&mut self) -> f64{
        match self.time_delta_root {
            None => {
                self.time_delta_root = Some(js_sys::Date::now());
                return 0.000000000000000000000000000001;
            },
            Some(time_delta_root) => {
                let now = js_sys::Date::now();
                let time_delta = ((now - time_delta_root))/(1000.0);
                self.time_delta_root = Some(now);
                return time_delta;
            }
        }
    }
}