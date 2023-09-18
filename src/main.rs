use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

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
    pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]];
        Self::do_run(root, &mut res);
        return res;
    }
    fn do_run(node: Option<Rc<RefCell<TreeNode>>>, pre: &mut Vec<Vec<i32>>) -> usize {
        if None.eq(&node) {
            return 0;
        }
        let node = node.unwrap().borrow_mut().as_ptr();
        unsafe {
            let left = Solution::do_run((*node).left.take(), pre);
            let mut idx = left;
            let right = Solution::do_run((*node).right.take(), pre);
            idx = idx.max(right)+1;
            if pre.len() - 1 < idx as usize {
                pre.push(vec![]);
            }
            pre[idx-1].push((*node).val);
            return idx;
        }
    }
}

fn main() {}