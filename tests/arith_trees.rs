use trees::tree_v2::BinaryTree;

// Want to parse expressions such as:
// 3 + 6
// 3 - 5 + 6
// 3 - 4 * 5
// (3 - 4) * 5

#[derive(Debug, PartialEq, Eq)]
enum ArithNode {
    Add,
    Subtract,
    Multiply,
    Power,
    Constant(i32),
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    ArithNode(ArithNode),
    OpenParen,
    CloseParen,
}

fn tokenize_string(arithmetic_experession: &str) -> Vec<Token> {
    panic!()
}

fn make_tree_from_string(arithmetic_experession: &str) -> BinaryTree<ArithNode> {
    let mut tokenized = tokenize_string(arithmetic_experession);

    panic!();   
}

#[test]
fn test_tokenize_string() {
    let s1 = "5 + 4";
    assert_eq!(tokenize_string(s1),
               vec![Token::ArithNode(ArithNode::Constant(5i32)),
                    Token::ArithNode(ArithNode::Add),
                    Token::ArithNode(ArithNode::Constant(4i32))]);
}
