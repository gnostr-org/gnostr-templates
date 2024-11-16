use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug)]
struct Node2<'a> {
    value: i32,
    parent: Option<&'a Node2<'a>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl<'a> Node2<'a> {
    fn set_parent(&mut self, parent: Option<&'a Node2<'a>>) {
        self.parent = parent
    }
}

pub(crate) fn tree() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let mut leaf2 = Node2 {
        value: 1337,
        parent: None,
        children: RefCell::new(vec![]),
    };

    let root = Node2 {
        value: 100,
        parent: None,
        //        children: RefCell::new(vec!(Rc:clone(&leaf2))),
        children: RefCell::new(vec![]),
    };

    leaf2.set_parent(Some(&root));

    println!("leaf: {:?}", leaf2);
    println!("root: {:?}", root);
}
