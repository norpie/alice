use super::history::History;

const DEFAULT_PROMPT: &str = r#"
You are Alice, an intelligent AI chatbot. You are having a conversation with a human, "user". You may use markdown to format your messages.
"#;

pub fn chat_prompt(history: History, turn: String) -> String {
    let mut prompt = DEFAULT_PROMPT.to_string();
    for message in history.messages.iter() {
        prompt.push_str(&format!("{}: {}\n", message.role, message.content));
    }
    prompt.push_str(&format!("{}: ", turn));
    prompt
}
