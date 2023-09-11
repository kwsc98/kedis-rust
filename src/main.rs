use std::{borrow::BorrowMut, cell::RefCell, collections::LinkedList, rc::Rc};
pub struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn boundary_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res1_linked_list = LinkedList::new();
        let mut res2 = vec![];

        if let None = root {
            return res2;
        }
        let mut count = 1;
        let mut linked_list = LinkedList::new();
        linked_list.push_back(root.unwrap());
        while !linked_list.is_empty() {
            let mut node = linked_list.pop_front().unwrap();
            let val = node.borrow().val;
            let node = node.borrow_mut().as_ptr();
            count -= 1;
            let mut end_pre = false;
            unsafe {
                if let Some(left) = (*node).left.take() {
                    end_pre = true;
                    linked_list.push_back(left);
                }
                if let Some(right) = (*node).right.take() {
                    end_pre = true;
                    linked_list.push_back(right);
                }
            }
            if count == 0 {
                if end_pre {
                    res1_linked_list.push_back(val);
                } else {
                    res2.push(val);
                    if !linked_list.is_empty(){
                        res1_linked_list.push_back(linked_list.front().unwrap().borrow().val);
                    }
                }
                count = linked_list.len();
            }
        }
        res1_linked_list.push_front(res2[0]);
        for idx in (0..res2.len() - 1).rev() {
            res1_linked_list.push_back(res2[idx]);
        }
        let mut res = vec![];
        while !res1_linked_list.is_empty() {
            res.push(res1_linked_list.pop_front().unwrap());
        }
        return res;
    }
}

fn main() {}
