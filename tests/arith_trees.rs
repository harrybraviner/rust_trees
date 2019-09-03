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

fn tokenize_string(arithmetic_experession: &str) -> Result<Vec<Token>, String> {
    let mut tokens : std::vec::Vec<Token> = vec![];
    let mut current_const_str_start : usize = 0;
    let mut currently_in_const = false;

    for (i, c) in arithmetic_experession.chars().enumerate() {
        if c.is_numeric() {
            // If this character is part of a numeric string, we need to wait until next char to
            // determine if we've parsed all of it. Just note the start of the string, if
            // necessary.
            if !currently_in_const {
                currently_in_const = true;
                current_const_str_start = i;
            }
        } else {
            // If the previous character was part of a const, we're done - close it.
            if currently_in_const {
                currently_in_const = false;
                let const_str = &arithmetic_experession[current_const_str_start .. i];
                let x : i32 = match const_str.parse() {
                    Ok(x) => x,
                    Err(_) => { return Result::Err(format!("Unable to parse {} as an i32 at position {}", const_str, current_const_str_start)) }
                };
                tokens.push(Token::ArithNode(ArithNode::Constant(x)));
            }

            match c {
                ' ' => {}, // Whitespace, don't have a token for this.
                '+' => tokens.push(Token::ArithNode(ArithNode::Add)),
                '-' => tokens.push(Token::ArithNode(ArithNode::Subtract)),
                '*' => tokens.push(Token::ArithNode(ArithNode::Multiply)),
                '^' => tokens.push(Token::ArithNode(ArithNode::Power)),
                '(' => tokens.push(Token::OpenParen),
                ')' => tokens.push(Token::CloseParen),
                _ => { return Result::Err(format!("Syntax error: unable to parse {} at position {}", c, i))}
            }
        }
    }
    // If the previous character was part of a const, we're done - close it.
    if currently_in_const {
        currently_in_const = false;
        let const_str = &arithmetic_experession[current_const_str_start .. arithmetic_experession.len()];
        let x : i32 = match const_str.parse() {
            Ok(x) => x,
            Err(_) => { return Result::Err(format!("Unable to parse {} as an i32 at position {}.", const_str, current_const_str_start)) }
        };
        tokens.push(Token::ArithNode(ArithNode::Constant(x)));
    }

    return Result::Ok(tokens);
}

fn make_tree_from_string(arithmetic_experession: &str) -> BinaryTree<ArithNode> {
    let mut tokenized = tokenize_string(arithmetic_experession);

    panic!();   
}

#[test]
fn test_tokenize_string() {
    let s1 = "5 + 4";
    assert_eq!(tokenize_string(s1),
               Result::Ok(vec![Token::ArithNode(ArithNode::Constant(5i32)),
                               Token::ArithNode(ArithNode::Add),
                               Token::ArithNode(ArithNode::Constant(4i32))]));

    let s2 = "5 6 7 - - (";
    assert_eq!(tokenize_string(s2),
               Result::Ok(vec![Token::ArithNode(ArithNode::Constant(5i32)),
                               Token::ArithNode(ArithNode::Constant(6i32)),
                               Token::ArithNode(ArithNode::Constant(7i32)),
                               Token::ArithNode(ArithNode::Subtract),
                               Token::ArithNode(ArithNode::Subtract),
                               Token::OpenParen]));

    let s3 = "( 1 + - * ^ )";
    assert_eq!(tokenize_string(s3),
               Result::Ok(vec![Token::OpenParen,
                               Token::ArithNode(ArithNode::Constant(1i32)),
                               Token::ArithNode(ArithNode::Add),
                               Token::ArithNode(ArithNode::Subtract),
                               Token::ArithNode(ArithNode::Multiply),
                               Token::ArithNode(ArithNode::Power),
                               Token::CloseParen]));
}
