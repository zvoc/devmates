// 09/09/2019
//
// Implement an iterator over a Binary Search Tree (BST). Your iterator will
// be initialized with the root node of a BST.
//
// Calling next() will return the next smallest number in the BST.
//
// Example:
//      7
//    /   \
//   3    15
//        / \
//       9   20
//
//  BSTIterator iterator = new BSTIterator(root);
//  iterator.next();    // return 3
//  iterator.next();    // return 7
//  iterator.hasNext(); // return true
//  iterator.next();    // return 9
//  iterator.hasNext(); // return true
//  iterator.next();    // return 15
//  iterator.hasNext(); // return true
//  iterator.next();    // return 20
//  iterator.hasNext(); // return false

struct BSTNode<T: Ord> {
    value: T,
    left: Option<Box<BSTNode<T>>>,
    right: Option<Box<BSTNode<T>>>
}

impl<T: Ord> BSTNode<T> {
    fn new(value: T) -> Self {
        BSTNode{value: value, left: None, right: None}
    }
}

struct BST<T: Ord> {
    head: Option<Box<BSTNode<T>>>
}

impl<T: Ord> BST<T> {
    fn new() -> Self {
        BST{head: None}
    }

    fn push(&mut self, value: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(
                    BSTNode{value: value, left: None, right: None}));
        } else {
            let mut node = self.head.as_mut().unwrap();
            loop {
                if value <= node.value {
                    if node.left.is_none() {
                        node.left = Some(Box::new(BSTNode::new(value)));
                        break;
                    } else {
                        node = node.left.as_mut().unwrap();
                        continue;
                    }
                }

                if value > node.value {
                    if node.right.is_none() {
                        node.right = Some(Box::new(BSTNode::new(value)));
                        break;
                    } else {
                        node = node.right.as_mut().unwrap();
                        continue;
                    }
                }
            }
        }
    }

    fn iter(&self) -> BSTIter<T> {
        BSTIter{
            action: Action::GoLeft,
            node: &self.head,
            ref_stack: Vec::new(),
            action_stack: Vec::new()
        }
    }
}

enum Action {
    GoLeft,
    ReturnValue,
    GoRight,
    GoUp,
}

struct BSTIter<'a, T: Ord> {
    action: Action,
    node: &'a Option<Box<BSTNode<T>>>,
    ref_stack: Vec<&'a Option<Box<BSTNode<T>>>>,
    action_stack: Vec<Action>
}

impl<T: Ord> BSTIter<'_, T> {
    fn next(&mut self) -> Option<&T> {
        if self.node.is_none() {
            return None;
        }

        loop {
            match self.action {
                Action::GoLeft => {
                    if self.node.as_ref().unwrap().left.is_some() {
                        self.action_stack.push(Action::ReturnValue);
                        self.ref_stack.push(self.node);
                        self.node = &self.node.as_ref().unwrap().left;

                    } else {
                        self.action = Action::ReturnValue;
                    }
                }

                Action::ReturnValue => {
                    self.action = Action::GoRight;
                    return Some(&self.node.as_ref().unwrap().value);
                }

                Action::GoRight => {
                    if self.node.as_ref().unwrap().right.is_some() {
                        self.action_stack.push(Action::GoUp);
                        self.ref_stack.push(self.node);
                        self.node = &self.node.as_ref().unwrap().right;
                        self.action = Action::GoLeft;

                    } else {
                        self.action = Action::GoUp;
                    }
                }

                Action::GoUp => {
                    if self.ref_stack.len() == 0 {
                        return None;
                    } else {
                        self.node = self.ref_stack.pop().unwrap();
                        self.action = self.action_stack.pop().unwrap();
                    }
                }
            }
        }
    }
}

fn main() {
    let mut tree = BST::new();
    tree.push(7);
    tree.push(3);
    tree.push(15);
    tree.push(9);
    tree.push(20);

    let mut iter = tree.iter();
    while let Some(&val) = iter.next() {
        println!("{}", val);
    }
}

