use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct SlackPostMessageBody {
    pub channel: String,
    pub text: String,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SlackPostMessageResponse {
    pub ok: bool,
    pub error: Option<String>,
}
