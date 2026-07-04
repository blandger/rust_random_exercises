// Дерево с родителем

// Реализуй структуру Node, которая представляет узел дерева:
// Требования:
// Узел может иметь родителя (Weak), и это опционально (у корня нет родителя).
// 
// Узел может иметь детей (Rc<RefCell<>>, чтобы разделять и мутировать).
// 
// Напиши функцию add_child(parent: &Rc<RefCell<Node>>, value: i32) -> Rc<RefCell<Node>>, которая:
// 
// создаёт нового потомка с заданным значением,
// 
// устанавливает ссылку на родителя (Weak),
// 
// добавляет потомка в список children.
#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    parent: Weak<RefCell<Node>>,
    children: Vec<Rc<RefCell<Node>>>,
}
impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            parent: Weak::new(),
            children: vec![],
        }
    }
}

fn add_child(parent: &Rc<RefCell<Node>>, value: i32) -> Rc<RefCell<Node>> {
    // создаёт нового потомка с заданным значением,
    let child = Rc::new(RefCell::new(Node::new(value)));
    // устанавливает ссылку на родителя (Weak),
    child.borrow_mut().parent = Rc::downgrade(parent);
    // добавляет потомка в список children.
    parent.borrow_mut().children.push(child.clone());
    child
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::*;

    #[test]
    fn test_tree_structure() {
        let root = Rc::new(RefCell::new(Node::new(1)));

        let child = add_child(&root, 2);

        assert_eq!(child.borrow().value, 2);
        assert_eq!(root.borrow().children.len(), 1);

        // Проверим родителя через Weak
        assert_eq!(
            child
                .borrow()
                .parent
                .upgrade()
                .unwrap()
                .borrow()
                .value,
            1
        );
    }
}