pub struct BinaryTree<T> {
    pub left_link: Option<Box<BinaryTree<T>>>,
    pub right_link: Option<Box<BinaryTree<T>>>,
    pub payload: T
}

impl<T> BinaryTree<T> {

    pub fn leaf(t: T) -> BinaryTree<T> {
        BinaryTree { left_link: None, right_link: None, payload: t }
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryTree;

    #[test]
    fn construct_single_node_tree() {
        BinaryTree::leaf(1i32);
    }

    #[test]
    fn construct_many_node_tree() {
        //     0
        //    / \
        //   3   4
        //  / \   \
        // 1   2   5
        BinaryTree {
            left_link: Some(Box::new(BinaryTree {
                left_link: Some(Box::new(BinaryTree::leaf(1))),
                right_link: Some(Box::new(BinaryTree::leaf(2))),
                payload: 3,
            })),
            right_link: Some(Box::new(BinaryTree {
                left_link: None,
                right_link: Some(Box::new(BinaryTree::leaf(5))),
                payload: 4,
            })),
            payload: 0i32
        };
    }
}

