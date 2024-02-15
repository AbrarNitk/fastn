#[derive(serde::Deserialize)]
pub struct UserData {
    pub username: String,
    pub name: String,
    pub email: String,
    pub verified_email: bool,
}
