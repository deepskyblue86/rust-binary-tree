use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct TreeNodePtr {
    pub ptr: Rc<RefCell<TreeNode>>,
}

impl Deref for TreeNodePtr {
    type Target = Rc<RefCell<TreeNode>>;
    fn deref(&self) -> &Rc<RefCell<TreeNode>> {
        &self.ptr
    }
}

pub type TreeNodeLink = Option<TreeNodePtr>;

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
            parent: None,
            left: None,
            right: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
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
        node_ptr.borrow_mut().parent = Some(self.clone());

        return node_ptr;
    }

    pub fn add_left(&mut self, value: u32) -> TreeNodePtr {
        let node_ptr = self.create_child(value);
        // set it as left child of `self`
        self.borrow_mut().left = Some(node_ptr.clone());

        return node_ptr;
    }

    pub fn add_right(&mut self, value: u32) -> TreeNodePtr {
        let node_ptr = self.create_child(value);
        // set it as right child of `self`
        self.borrow_mut().right = Some(node_ptr.clone());

        return node_ptr;
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
        if let Some(left) = &node.borrow_mut().left {
            Tree::visit_node(&left);
        }

        print!("{}, ", node.borrow_mut().value);

        if let Some(right) = &node.borrow_mut().right {
            Tree::visit_node(&right);
        }
    }

    pub fn visit(&self) {
        Tree::visit_node(&self.root);
        println!("");
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
}
