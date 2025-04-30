pub struct Session {
    user_id: Option<i32>,
    user_name: Option<String>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            user_id: None,
            user_name: None,
        }
    }

    pub fn login(&mut self) {}
}
