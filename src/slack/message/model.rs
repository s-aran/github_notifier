use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SlackPostMessageBlocksText {
    #[serde(rename = "type")]
    pub text_type: String,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct SlackPostMessageBlocks {
    #[serde(rename = "type")]
    pub block_type: String,
    pub text: SlackPostMessageBlocksText,
}

#[derive(Debug, Serialize)]
pub struct SlackPostMessageBody {
    pub channel: String,
    // #[serde(rename = "blocks")]
    // pub text: String,
    pub blocks: Vec<SlackPostMessageBlocks>,
    pub username: Option<String>,
    // pub mrkdwn: bool,
}

#[derive(Debug, Deserialize)]
pub struct SlackPostMessageResponse {
    pub ok: bool,
    pub error: Option<String>,
}
