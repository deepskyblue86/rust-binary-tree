use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub struct TreeNodePtr {
    pub ptr: Rc<RefCell<TreeNode>>,
}

impl Deref for TreeNodePtr {
    type Target = Rc<RefCell<TreeNode>>;
    fn deref(&self) -> &Rc<RefCell<TreeNode>> {
        &self.ptr
    }
}

#[derive(Clone)]
pub enum TreeNodeLink {
    Nil,
    Ptr(TreeNodePtr),
}

pub struct TreeNode {
    pub value: u32,
    pub parent: TreeNodeLink,
    pub left: TreeNodeLink,
    pub right: TreeNodeLink,
}

impl TreeNode {
    pub fn new(value: u32) -> TreeNode {
        TreeNode {
            value,
            parent: TreeNodeLink::Nil,
            left: TreeNodeLink::Nil,
            right: TreeNodeLink::Nil,
        }
    }

    pub fn has_left(&self) -> bool {
        match &self.left {
            TreeNodeLink::Nil => false,
            TreeNodeLink::Ptr(_) => true,
        }
    }
    pub fn has_right(&self) -> bool {
        match &self.right {
            TreeNodeLink::Nil => false,
            TreeNodeLink::Ptr(_) => true,
        }
    }
    pub fn is_leaf(&self) -> bool {
        !self.has_left() && !self.has_right()
    }
}

impl TreeNodePtr {
    fn new(value: u32) -> TreeNodePtr {
        TreeNodePtr {
            ptr: Rc::new(RefCell::new(TreeNode::new(value))),
        }
    }
    fn clone(&self) -> TreeNodePtr {
        TreeNodePtr {
            ptr: Rc::clone(&self.ptr),
        }
    }

    fn create_child(&mut self, value: u32) -> TreeNodePtr {
        // create the node
        let node_ptr = TreeNodePtr::new(value);
        // set `self` as its parent
        node_ptr.borrow_mut().parent = TreeNodeLink::Ptr(self.clone());

        return node_ptr;
    }

    pub fn add_left(&mut self, value: u32) -> TreeNodePtr {
        let node_ptr = self.create_child(value);
        // set it as left child of `self`
        self.borrow_mut().left = TreeNodeLink::Ptr(node_ptr.clone());

        return node_ptr;
    }

    pub fn add_right(&mut self, value: u32) -> TreeNodePtr {
        let node_ptr = self.create_child(value);
        // set it as right child of `self`
        self.borrow_mut().right = TreeNodeLink::Ptr(node_ptr.clone());

        return node_ptr;
    }

    pub fn get_left(&self) -> TreeNodeLink {
        match &self.borrow_mut().left {
            TreeNodeLink::Nil => TreeNodeLink::Nil,
            TreeNodeLink::Ptr(left) => TreeNodeLink::Ptr(left.clone()),
        }
    }
    pub fn get_right(&self) -> TreeNodeLink {
        match &self.borrow_mut().right {
            TreeNodeLink::Nil => TreeNodeLink::Nil,
            TreeNodeLink::Ptr(right) => TreeNodeLink::Ptr(right.clone()),
        }
    }
    pub fn get_parent(&self) -> TreeNodeLink {
        match &self.borrow_mut().parent {
            TreeNodeLink::Nil => TreeNodeLink::Nil,
            TreeNodeLink::Ptr(parent) => TreeNodeLink::Ptr(parent.clone()),
        }
    }
}

impl std::fmt::Display for TreeNodePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.borrow_mut().value)
    }
}

struct Tree {
    root: TreeNodePtr,
}

impl Tree {
    pub fn new(value: u32) -> Tree {
        Tree {
            root: TreeNodePtr::new(value),
        }
    }

    pub fn add_left(&mut self, value: u32) -> TreeNodePtr {
        self.root.add_left(value)
    }
    pub fn add_right(&mut self, value: u32) -> TreeNodePtr {
        self.root.add_right(value)
    }

    fn visit_node(node: &TreeNodePtr) {
        if let TreeNodeLink::Ptr(left) = &node.borrow_mut().left {
            Tree::visit_node(&left);
        }

        print!("{}, ", node.borrow_mut().value);

        if let TreeNodeLink::Ptr(right) = &node.borrow_mut().right {
            Tree::visit_node(&right);
        }
    }

    pub fn visit(&self) {
        Tree::visit_node(&self.root);
        println!("");
    }

    pub fn iter(&self) -> TreeIter {
        TreeIter::new(&self)
    }
}

#[derive(Clone, Copy)]
enum Movement {
    UNKNOWN,
    LEFT,
    RIGHT,
    UP,
}

impl Default for Movement {
    fn default() -> Self {
        Movement::UNKNOWN
    }
}

#[derive(Clone)]
struct TreeIter {
    curr: TreeNodeLink,
    last_move: Movement,
    ascended: bool,
}

impl Default for TreeIter {
    fn default() -> Self {
        Self {
            curr: TreeNodeLink::Nil,
            last_move: Movement::UNKNOWN,
            ascended: false,
        }
    }
}

impl TreeIter {
    pub fn new(t: &Tree) -> TreeIter {
        let mut it = TreeIter::default();
        it.curr = TreeNodeLink::Ptr(t.root.clone());

        it.descend();

        return it;
    }

    fn go_left(&mut self) {
        if let TreeNodeLink::Ptr(curr) = &self.curr {
            if let TreeNodeLink::Ptr(left) = curr.get_left() {
                self.curr = TreeNodeLink::Ptr(left.clone());
                self.last_move = Movement::LEFT;
            }
        }
    }
    fn go_right(&mut self) {
        if let TreeNodeLink::Ptr(curr) = &self.curr {
            if let TreeNodeLink::Ptr(right) = curr.get_right() {
                self.curr = TreeNodeLink::Ptr(right.clone());
                self.last_move = Movement::RIGHT;
            }
        }
    }
    fn go_up(&mut self) {
        if let TreeNodeLink::Ptr(curr) = &self.curr {
            if let TreeNodeLink::Ptr(parent) = curr.get_parent() {
                self.curr = TreeNodeLink::Ptr(parent.clone());
                self.last_move = Movement::UP;
            }
        }
    }

    fn descend(&mut self) {
        if let TreeNodeLink::Ptr(curr) = &self.curr {
            let mut curr = curr.clone();

            while let TreeNodeLink::Ptr(_) = curr.get_left() {
                self.go_left();

                if let TreeNodeLink::Ptr(updated_curr) = &self.curr {
                    curr = updated_curr.clone();
                }
            }
        }
    }
    fn ascend(&mut self) {
        if let TreeNodeLink::Ptr(curr) = &self.curr {
            let mut curr = curr.clone();

            while let TreeNodeLink::Ptr(_) = curr.get_parent() {
                self.go_up();

                if let TreeNodeLink::Ptr(updated_curr) = &self.curr {
                    curr = updated_curr.clone();
                }
            }

            self.ascended = true;
        }
    }
}

impl Iterator for TreeIter {
    type Item = TreeNodePtr;

    fn next(&mut self) -> Option<TreeNodePtr> {
        // Here I had to clone self because otherwise the borrow checker complains
        // when I try to use go_* methods
        match self.clone().curr {
            TreeNodeLink::Nil => None,
            TreeNodeLink::Ptr(curr) => {
                match &self.last_move {
                    Movement::LEFT => {
                        self.go_up();
                    }
                    Movement::UP => {
                        if let TreeNodeLink::Ptr(_) = curr.get_right() {
                            self.go_right();
                            self.descend();
                        } else {
                            self.curr = TreeNodeLink::Nil;
                        }
                    }
                    Movement::RIGHT => {
                        if let TreeNodeLink::Ptr(_) = curr.get_left() {
                            self.descend();
                        } else if let TreeNodeLink::Ptr(_) = curr.get_right() {
                            self.go_right();
                            self.descend();
                        } else {
                            if self.ascended {
                                self.curr = TreeNodeLink::Nil;
                            } else {
                                self.ascend();
                            }
                        }
                    }
                    Movement::UNKNOWN => {}
                }

                Some(curr.clone())
            }
        }
    }
}

fn main() {
    /* https://upload.wikimedia.org/wikipedia/commons/thumb/d/da/Binary_search_tree.svg/2560px-Binary_search_tree.svg.png
          8
         / \
        3   10
       / \    \
      1   6    14
         / \   /
        4   7 13
    */
    let mut t = Tree::new(8);

    let mut node3 = t.add_left(3);
    node3.add_left(1);
    let mut node6 = node3.add_right(6);
    node6.add_left(4);
    node6.add_right(7);

    t.add_right(10).add_right(14).add_left(13);

    t.visit();

    println!("Iterator visit");

    for n in t.iter() {
        print!("{}, ", n);
    }
}
