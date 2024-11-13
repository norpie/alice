use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Author {
    RegisteredAuthor { id: Uuid },
    OneOffCharacter { name: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SimpleMessage {
    id: Uuid,
    time: DateTime<Utc>,
    edit_time: Option<DateTime<Utc>>,
    author: Author,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SimpleHistory {
    messages: Vec<SimpleMessage>,
}

#[derive(Debug, Clone)]
struct MessageNode {
    id: Uuid,
    author: Author,
    time: DateTime<Utc>,
    content: String,
    deleted: bool,
    parent: Option<Weak<RefCell<MessageNode>>>,
    children: Vec<Rc<RefCell<MessageNode>>>,
}

impl MessageNode {
    fn new(
        content: String,
        author: Author,
        id: Uuid,
        parent: Option<Weak<RefCell<MessageNode>>>,
    ) -> Rc<RefCell<MessageNode>> {
        Rc::new(RefCell::new(MessageNode {
            content,
            author,
            time: Utc::now(),
            id,
            deleted: false,
            parent,
            children: Vec::new(),
        }))
    }
}

// Enum to track actions for undo functionality
#[derive(Serialize, Deserialize, Debug, Clone)]
enum Action {
    Edit {
        id: Uuid,
        old_content: String,
        time: DateTime<Utc>,
    },
    Delete {
        id: Uuid,
    },
}

// Chat history tree structure
#[derive(Debug, Clone)]
struct ChatHistoryTree {
    root: Rc<RefCell<MessageNode>>,
    current: Rc<RefCell<MessageNode>>,
    action_history: VecDeque<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveableMessage {
    id: Uuid,
    parent: Option<Uuid>,
    author: Author,
    time: DateTime<Utc>,
    content: String,
    deleted: bool,
}

// Meant to be saved to a database, etc. We need to make sure that we can build a tree from this
// structure. That means we should save: all the messages, their parents, and the action history.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveableChatHistoryTree {
    // Root message ID
    root: Uuid,
    messages: Vec<SaveableMessage>,
    action_history: Vec<Action>,
}

impl From<ChatHistoryTree> for SaveableChatHistoryTree {
    fn from(val: ChatHistoryTree) -> Self {
        let mut messages = Vec::new();
        let mut current = Rc::clone(&val.current);

        loop {
            let current_borrow = current.borrow();
            messages.push(SaveableMessage {
                id: current_borrow.id,
                parent: current_borrow
                    .parent
                    .as_ref()
                    .map(|p| p.upgrade().unwrap().borrow().id),
                author: current_borrow.author.clone(),
                time: current_borrow.time,
                content: current_borrow.content.clone(),
                deleted: current_borrow.deleted,
            });
            if let Some(parent) = current_borrow.parent.as_ref().and_then(|p| p.upgrade()) {
                drop(current_borrow); // Explicitly drop the borrow before moving to the parent
                current = parent;
            } else {
                break;
            }
        }

        SaveableChatHistoryTree {
            root: val.root.borrow().id,
            messages,
            action_history: val.action_history.clone().into(),
        }
    }
}

impl ChatHistoryTree {
    pub fn new(initial_content: String, author: Author) -> Self {
        let root = MessageNode::new(initial_content, author, Uuid::new_v4(), None);
        ChatHistoryTree {
            root: Rc::clone(&root), // Clone here to avoid moving `root`
            current: root,
            action_history: VecDeque::new(),
        }
    }

    pub fn add_message(&mut self, content: String, author: Author) -> Uuid {
        let uuid = Uuid::new_v4();
        let new_message =
            MessageNode::new(content, author, uuid, Some(Rc::downgrade(&self.current)));

        // Drop the borrow of `self.current` before accessing its children
        let new_current = {
            let mut current_borrow = self.current.borrow_mut();
            current_borrow.children.push(new_message);
            Rc::clone(current_borrow.children.last().unwrap()).clone()
        };
        // Update current to the new message
        self.current = new_current;
        uuid
    }

    fn find_message_by_id(&self, id: Uuid) -> Option<Rc<RefCell<MessageNode>>> {
        Self::find_message_recursive(&self.root, id)
    }

    fn find_message_recursive(
        node: &Rc<RefCell<MessageNode>>,
        id: Uuid,
    ) -> Option<Rc<RefCell<MessageNode>>> {
        let node_ref = node.borrow();
        if node_ref.id == id {
            return Some(Rc::clone(node));
        }
        for child in &node_ref.children {
            if let Some(found) = Self::find_message_recursive(child, id) {
                return Some(found);
            }
        }
        None
    }

    // Edit message by ID
    pub fn edit_message_by_id(&mut self, id: Uuid, new_content: String) -> Result<(), String> {
        if let Some(node) = self.find_message_by_id(id) {
            let old_content = {
                let node_ref = node.borrow();
                node_ref.content.clone()
            };
            node.borrow_mut().content = new_content;
            self.action_history.push_back(Action::Edit {
                id,
                old_content,
                time: Utc::now(),
            });
            Ok(())
        } else {
            Err(format!("Message with ID {} not found.", id))
        }
    }

    // Delete message by ID
    pub fn delete_message_by_id(&mut self, id: Uuid) -> Result<(), String> {
        if let Some(node) = self.find_message_by_id(id) {
            let mut node_ref = node.borrow_mut();
            if !node_ref.deleted {
                node_ref.deleted = true;
                self.action_history.push_back(Action::Delete { id });
                Ok(())
            } else {
                Err("Message already deleted.".to_string())
            }
        } else {
            Err(format!("Message with ID {} not found.", id))
        }
    }

    // Unified undo method
    pub fn undo(&mut self) -> Result<(), String> {
        if let Some(action) = self.action_history.pop_back() {
            match action {
                Action::Edit {
                    id,
                    old_content,
                    time: _,
                } => {
                    if let Some(node) = self.find_message_by_id(id) {
                        node.borrow_mut().content = old_content; // Restore old content
                        Ok(())
                    } else {
                        Err(format!("Message with ID {} not found.", id))
                    }
                }
                Action::Delete { id } => {
                    if let Some(node) = self.find_message_by_id(id) {
                        let mut node_ref = node.borrow_mut();
                        if node_ref.deleted {
                            node_ref.deleted = false; // Undo the deletion
                            Ok(())
                        } else {
                            Err(format!("Message with ID {} is not deleted.", id))
                        }
                    } else {
                        Err(format!("Message with ID {} not found.", id))
                    }
                }
            }
        } else {
            Err("No actions to undo.".to_string())
        }
    }

    // Display current state with author information
    pub fn simple_history(&self) -> SimpleHistory {
        let mut messages = Vec::new();
        let mut current = Rc::clone(&self.current);
        loop {
            let current_borrow = current.borrow();
            if !current_borrow.deleted {
                let most_recent_edit_time: Option<DateTime<Utc>> = self
                    .action_history
                    .iter()
                    .filter_map(|action| match action {
                        Action::Edit {
                            id,
                            old_content: _,
                            time,
                        } if *id == current_borrow.id => Some(*time),
                        _ => None,
                    })
                    .max();
                messages.push(SimpleMessage {
                    content: current_borrow.content.clone(),
                    author: current_borrow.author.clone(),
                    time: current_borrow.time,
                    edit_time: most_recent_edit_time,
                    id: current_borrow.id,
                });
            }
            if let Some(parent) = current_borrow.parent.as_ref().and_then(|p| p.upgrade()) {
                drop(current_borrow); // Explicitly drop the borrow before moving to the parent
                current = parent;
            } else {
                break;
            }
        }
        messages.reverse();
        SimpleHistory { messages }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_message() {
        let mut chat = ChatHistoryTree::new(
            "Welcome to the chat.".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        println!("Initial state:");
        let history = chat.simple_history();
        assert_eq!(history.messages.len(), 1);
        println!("{:?}", history.messages);

        // Add a message
        chat.add_message(
            "Hello from Alice!".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );
        println!("State after adding Alice's message:");
        let history = chat.simple_history();
        assert_eq!(history.messages.len(), 2);
        println!("{:?}", history.messages);
    }

    #[test]
    fn test_edit_message() {
        let mut chat = ChatHistoryTree::new(
            "Welcome to the chat.".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        let message_id = chat.add_message(
            "Hello from Alice!".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        println!("State before editing:");
        let history = chat.simple_history();
        println!("{:?}", history.messages);

        // Edit the message
        chat.edit_message_by_id(message_id, "Hello from Alice (edited)!".to_string())
            .unwrap();

        println!("State after editing:");
        let history = chat.simple_history();
        assert_eq!(
            history.messages.last().unwrap().content,
            "Hello from Alice (edited)!"
        );
        println!("{:?}", history.messages);
    }

    #[test]
    fn test_delete_message() {
        let mut chat = ChatHistoryTree::new(
            "Welcome to the chat.".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        let message_id = chat.add_message(
            "Hello from Alice!".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        // Delete the message
        chat.delete_message_by_id(message_id).unwrap();

        println!("State after deletion:");
        let history = chat.simple_history();
        assert_eq!(history.messages.len(), 1); // Only the root message should remain
        println!("{:?}", history.messages);
    }

    #[test]
    fn test_undo_edit() {
        let mut chat = ChatHistoryTree::new(
            "Welcome to the chat.".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        let message_id = chat.add_message(
            "Hello from Alice!".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        // Edit the message
        chat.edit_message_by_id(message_id, "Hello from Alice (edited)!".to_string())
            .unwrap();
        println!("State after editing:");
        let history = chat.simple_history();
        println!("{:?}", history.messages);

        // Undo the edit
        chat.undo().unwrap();

        println!("State after undoing the edit:");
        let history = chat.simple_history();
        assert_eq!(
            history.messages.last().unwrap().content,
            "Hello from Alice!"
        );
        println!("{:?}", history.messages);
    }

    #[test]
    fn test_undo_delete() {
        let mut chat = ChatHistoryTree::new(
            "Welcome to the chat.".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        let message_id = chat.add_message(
            "Hello from Alice!".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        // Delete the message
        chat.delete_message_by_id(message_id).unwrap();

        println!("State after deletion:");
        let history = chat.simple_history();
        assert_eq!(history.messages.len(), 1); // Only the root message should remain
        println!("{:?}", history.messages);

        // Undo the deletion
        chat.undo().unwrap();

        println!("State after undoing the deletion:");
        let history = chat.simple_history();
        assert_eq!(history.messages.len(), 2); // The deleted message should be restored
        println!("{:?}", history.messages);
    }

    #[test]
    fn test_multiple_actions() {
        let mut chat = ChatHistoryTree::new(
            "Welcome to the chat.".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        let message_id = chat.add_message(
            "Hello from Alice!".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        let _ = chat.add_message(
            "A mysterious stranger appears.".to_string(),
            Author::OneOffCharacter {
                name: "Stranger".to_string(),
            },
        );

        println!("State before multiple actions:");
        let history = chat.simple_history();
        println!("{:?}", history.messages);

        // Edit Alice's message
        chat.edit_message_by_id(message_id, "Hello from Alice (edited)!".to_string())
            .unwrap();

        // Delete the stranger's message
        chat.delete_message_by_id(message_id).unwrap();

        println!("State after multiple actions:");
        let history = chat.simple_history();
        println!("{:?}", history.messages);

        chat.undo().unwrap(); // Undo delete
        println!("State after undoing delete:");
        let history = chat.simple_history();
        println!("{:?}", history.messages);

        chat.undo().unwrap(); // Undo edit
        println!("State after undoing all actions:");
        let history = chat.simple_history();
        println!("{:?}", history.messages);
    }

    #[test]
    fn test_undo_no_actions() {
        let mut chat = ChatHistoryTree::new(
            "Welcome to the chat.".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        // Try to undo with no actions
        let result = chat.undo();
        assert_eq!(result.clone().unwrap_err(), "No actions to undo.");
        println!(
            "Undoing with no actions gave expected error: {}",
            result.unwrap_err()
        );
    }

    #[test]
    fn test_edit_nonexistent_message() {
        let mut chat = ChatHistoryTree::new(
            "Welcome to the chat.".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        // Attempt to edit a nonexistent message
        let id = Uuid::new_v4();
        let result = chat.edit_message_by_id(id, "Nonexistent".to_string());
        assert_eq!(
            result.clone().unwrap_err(),
            format!("Message with ID {} not found.", id)
        );
        println!(
            "Editing nonexistent message gave expected error: {}",
            result.unwrap_err()
        );
    }

    #[test]
    fn test_delete_nonexistent_message() {
        let mut chat = ChatHistoryTree::new(
            "Welcome to the chat.".to_string(),
            Author::RegisteredAuthor { id: Uuid::new_v4() },
        );

        // Attempt to delete a nonexistent message
        let id = Uuid::new_v4();
        let result = chat.delete_message_by_id(id);
        assert_eq!(
            result.clone().unwrap_err(),
            format!("Message with ID {} not found.", id)
        );
        println!(
            "Deleting nonexistent message gave expected error: {}",
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into_saveable() {
        let authors = [
            Author::RegisteredAuthor { id: Uuid::new_v4() },
            Author::OneOffCharacter {
                name: "Stranger".to_string(),
            },
        ];
        let mut chat = ChatHistoryTree::new("Welcome to the chat.".to_string(), authors[0].clone());

        let root_id = chat.add_message("Hello from Alice!".to_string(), authors[0].clone());
        chat.add_message(
            "A mysterious stranger appears.".to_string(),
            authors[1].clone(),
        );

        chat.edit_message_by_id(root_id, "Hello from Alice (edited)!".to_string())
            .unwrap();

        chat.edit_message_by_id(root_id, "Hello from Alice (edited again)!".to_string())
            .unwrap();

        let saveable = SaveableChatHistoryTree::from(chat.clone());
        println!("{:#?}", saveable);

        let simple_history = chat.simple_history();
        println!("{:#?}", simple_history);
    }
}
