use std::{cell::RefCell, rc::Rc};
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
    pub fn longest_consecutive(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 1;
        Solution::do_run(root.as_mut(), &mut res);
        return res;
    }

    fn do_run(node: Option<&mut Rc<RefCell<TreeNode>>>, pre: &mut i32) -> (i32, i32, i32) {
        let mut res = (0, 0, 0);
        if let Some(node) = node {
            res.0 = node.borrow().val;
            if let Some(left_node) = node.borrow_mut().left.as_mut() {
                let temp = Solution::do_run(Some(left_node), pre);
                if res.0 - 1 == temp.0 {
                    res.1 = temp.1;
                }
                if res.0 + 1 == temp.0 {
                    res.2 = temp.2;
                }
            }
            if let Some(right_node) = node.borrow_mut().right.as_mut() {
                let temp = Solution::do_run(Some(right_node), pre);
                if res.0 - 1 == temp.0 {
                    res.1 = res.1.max(temp.1);
                }
                if res.0 + 1 == temp.0 {
                    res.2 = res.2.max(temp.2);
                }
            }
        }
        *pre = (res.1 + res.2 + 1).max(*pre);
        res.0 += 1;
        res.1 += 1;
        return res;
    }
}

fn main() {}
