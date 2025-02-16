use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize)]
pub struct CheckEmailOutputResponse {
    pub input: String,
    pub is_reachable: bool,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct CheckEmailRequest {
    #[validate(email)]
    pub email: String,
}
