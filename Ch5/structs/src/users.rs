pub struct Abuser {
    pub active: bool,
    pub username: String,
    pub location: String,
    pub sign_in_count: u128,
}

pub fn build_abuser(location: String, username: String) -> Abuser {
    Abuser {
        active: true,
        username,
        location,
        sign_in_count: 1,
    }
}
