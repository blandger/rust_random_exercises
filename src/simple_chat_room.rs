// Реализуй ChatRoom, где:
//
// У ChatRoom есть общий messages: Vec<String>.
//
// Несколько User могут одновременно писать туда сообщения.
//
// Каждый User имеет клон Rc<RefCell<Vec<String>>>.
//
// Метод send_message(&self, message: &str) у юзера добавляет сообщение в чат.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct User {
    messages: Rc<RefCell<Vec<String>>>,
}
impl User {
    pub fn send_message(&self, message: &str) {
        self.messages.borrow_mut().push(message.into());
    }
}

#[derive(Debug)]
struct ChatRoom {
    messages: Rc<RefCell<Vec<String>>>,
}
impl ChatRoom {
    pub fn new() -> Self {
        Self {
            messages: Rc::new(RefCell::new(Vec::new())),
        }
    }
    pub fn create_user(&self) -> User {
        User { 
            messages: self.messages.clone() 
        }
    }
    pub fn messages(&self) -> Vec<String> {
        self.messages.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_room() {
        let room = ChatRoom::new();
        let user1 = room.create_user();
        let user2 = room.create_user();

        user1.send_message("Hello from user1");
        user2.send_message("Hi from user2");

        let messages = room.messages();
        assert_eq!(messages, vec!["Hello from user1", "Hi from user2"]);
    }
}
