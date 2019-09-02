pub struct BinaryTree<'a> {
    left_link : Option<&'a BinaryTree<'a>>,
    right_link: Option<&'a BinaryTree<'a>>,
    payload: &'a i32,
}

impl BinaryTree<'_> {
    pub fn inorder_travsersal(&self, action: &mut dyn FnMut(&i32)) {
        match self.left_link {
            Some(n) => n.inorder_travsersal(action),
            _ => {},
        }
        action(self.payload);
        match self.right_link {
            Some(n) => n.inorder_travsersal(action),
            _ => {},
        }
    }

    pub fn from_children<'a>(left_link: Option<&'a BinaryTree>, right_link: Option<&'a BinaryTree>, payload: &'a i32) -> BinaryTree<'a> {
        BinaryTree {
            left_link: left_link,
            right_link: right_link,
            payload,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryTree;

    #[test]
    fn construct_single_node() {
        let _single_node = BinaryTree { left_link: None, right_link: None, payload: &0i32 };
    }

    #[test]
    fn construct_non_empty_left() {
        let _two_nodes = BinaryTree {
            left_link: None,
            right_link: Some(&BinaryTree{left_link: None, right_link: None, payload: &1i32}),
            payload: &0i32
        };
    }

    #[test]
    fn test_inorder_traversal() {
        let mut traversal_record : Vec<i32> = vec![];

        let mut store_traversal = |x: &i32| { traversal_record.push(x.clone()) };

        // many_nodes is this tree:
        //         0
        //        / \
        //       4   2
        //      /   / \
        //     3   1   5
        let many_nodes = BinaryTree {
            left_link: Some(&BinaryTree{
                left_link: Some(&BinaryTree{
                    left_link: None,
                    right_link: None,
                    payload: &3i32,
                }),
                right_link: None,
                payload: &4i32,
            }),
            right_link: Some(&BinaryTree{
                left_link: Some(&BinaryTree{
                    left_link: None,
                    right_link: None,
                    payload: &1i32,
                }),
                right_link: Some(&BinaryTree {
                    left_link: None,
                    right_link: None,
                    payload: &5i32
                }),
                payload: &2i32,
            }),
            payload: &0i32
        };

        many_nodes.inorder_travsersal(&mut store_traversal);

        assert_eq!(traversal_record, vec![3i32, 4i32, 0i32, 1i32, 2i32, 5i32]);
    }

    #[test]
    fn construct_in_stages() {
        let t1 = BinaryTree { left_link: None, right_link: None, payload: &1i32 };
        let t2 = BinaryTree { left_link: None, right_link: None, payload: &2i32 };
        let _t3 = BinaryTree { left_link: Some(&t1), right_link: Some(&t2), payload: &0i32 };
    }

    #[test]
    fn call_fn_that_briefly_makes_tree() {
        // Goal here is to provoke a problem by having references that live longer than the tree
        // holding them.
        
        fn use_trees(left_tree: BinaryTree, right_tree: BinaryTree) {
            
            let t = BinaryTree {
                left_link: Some(& left_tree),
                right_link: Some(& right_tree),
                payload: &0i32,
            };

            t.inorder_travsersal(&mut |_| {});    // Do nothing.
        }

        let t1 = BinaryTree { left_link: None, right_link: None, payload: &1i32 };
        let t2 = BinaryTree { left_link: None, right_link: None, payload: &2i32 };

        use_trees(t1, t2);
    }
}
