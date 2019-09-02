struct BinaryTree<'a> {
    left_link : Option<&'a BinaryTree<'a>>,
    right_link: Option<&'a BinaryTree<'a>>,
    payload: &'a i32,
}

#[cfg(test)]
mod tests {
    use super::BinaryTree;

    #[test]
    fn construct_single_node() {
        let single_node = BinaryTree { left_link: None, right_link: None, payload: &0i32 };
    }

    #[test]
    fn construct_non_empty_left() {
        let single_node = BinaryTree {
            left_link: None,
            right_link: Some(&BinaryTree{left_link: None, right_link: None, payload: &1i32}),
            payload: &0i32
        };
    }
}
