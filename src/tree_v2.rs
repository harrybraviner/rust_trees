pub struct BinaryTree<T> {
    pub left_link: Option<Box<BinaryTree<T>>>,
    pub right_link: Option<Box<BinaryTree<T>>>,
    pub payload: T
}

impl<T> BinaryTree<T> {

    pub fn leaf(t: T) -> BinaryTree<T> {
        BinaryTree { left_link: None, right_link: None, payload: t }
    }

    pub fn with_left_child(left_child: BinaryTree<T>, payload: T) -> BinaryTree<T> {
        BinaryTree { left_link: Some(Box::new(left_child)), right_link: None, payload: payload }
    }

    pub fn with_right_child(right_child: BinaryTree<T>, payload: T) -> BinaryTree<T> {
        BinaryTree { left_link: None, right_link: Some(Box::new(right_child)), payload: payload }
    }

    pub fn with_two_children(left_child: BinaryTree<T>, right_child: BinaryTree<T>, payload: T) -> BinaryTree<T> {
        BinaryTree { left_link: Some(Box::new(left_child)), right_link: Some(Box::new(right_child)), payload: payload }
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

    #[test]
    fn construct_many_node_tree_terse() {
        // Same as previous test, but hopefuly more terse.
        //     0
        //    / \
        //   3   4
        //  / \   \
        // 1   2   5
        BinaryTree::with_two_children(
            BinaryTree::with_two_children(
                BinaryTree::leaf(1i32),
                BinaryTree::leaf(2i32),
                3i32
            ),
            BinaryTree::with_right_child(
                BinaryTree::leaf(5i32),
                4i32
            ),
            0i32
        );
    }
}

