use anyhow::Result;
use std::rc::Rc;

use super::item::WppItem;

pub struct Message {
    author: Rc<WppItem>,
    message: String,
}

pub struct Example {
    dialogue: Vec<Message>,
}

pub struct ChatPrompt {
    characters: Vec<Rc<WppItem>>,
    user: Rc<WppItem>,
    first_message: Option<Message>,
    scenario: String,
    examples: Vec<Example>,
}

pub struct ChatPromptContext {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_prompt() {
        let example_char = Rc::new(WppItem::try_from(super::super::CHARACTER_FULL_VALID).unwrap());
        let char = Rc::clone(&example_char);
        let user = Rc::clone(&example_char);
        let characters = vec![char.clone()];
        // Some example messages
        let chat = ChatPrompt {
            characters,
            user: user.clone(),
            first_message: Some(Message {
                author: char.clone(),
                message: "Hello!".to_string(),
            }),
            scenario: "{{char[0].name}} meets {{user.name}}".to_string(),
            examples: vec![
                Example {
                    dialogue: vec![
                        Message {
                            author: char.clone(),
                            message: "Hello!".to_string(),
                        },
                        Message {
                            author: user.clone(),
                            message: "Hi!".to_string(),
                        },
                    ],
                },
                Example {
                    dialogue: vec![
                        Message {
                            author: char.clone(),
                            message: "How are you?".to_string(),
                        },
                        Message {
                            author: user.clone(),
                            message: "Good, you?".to_string(),
                        },
                    ],
                },
            ],
        };
        println!("{}", chat.prompt().unwrap());
    }
}
