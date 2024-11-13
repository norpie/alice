use super::message::Message;

pub struct Chat {
    pub id: String,
    pub name: String,
    pub messages: Vec<Message>,
}
