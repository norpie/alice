use crate::chat::History;

pub fn chat_prompt(history: History, turn: String) -> String {
    let mut prompt = "You are Alice, an intelligent AI chatbot.".to_string();
    for message in history.messages.iter().rev() {
        prompt.push_str(&format!("{}: {}\n", message.author, message.content));
    }
    prompt.push_str(&format!("{}: ", turn));
    prompt
}
