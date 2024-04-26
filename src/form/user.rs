use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct RegisterFormData {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct LoginFormData {
    pub email: String,
    pub password: String,
}
