extern crate serde_derive;
use self::serde_derive::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub content: String,
    pub mode: Mode,
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub og_msg: Message,
    pub response: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Mode {
    CHAT,
    CMD,
}
