#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub password_hash: String,
    pub email: String,
}
