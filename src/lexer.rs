#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Keywords
    Class,
    Else,
    False,
    For,
    If,
    Nil,
    Return,
    True,
    Var,
    While,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Semicolon,
    Comma,
    OpenParenthesis,
    CloseParenthesis,
    EndOfFile,
    Error(String),
}

use std::error::Error;

fn skip_whitespace(src: &str) -> usize {
    match src
        .char_indices()
        .take_while(|ch| (ch.1).is_ascii_whitespace())
        .last()
    {
        Some(x) => x.0 + 1,
        None => 0,
    }
}

fn skip_comment(src: &str) -> usize {
    if src.starts_with("//") {
        // skip past the terminating newline
        if let Some(x) = src.find('\n') {
            x + 1
        }
        // no newline terminator. consume entire line
        else {
            src.len()
        }
    } else {
        0
    }
}

fn skip(src: &str) -> usize {
    let mut remaining = src;

    loop {
        let whitespace_skipped = skip_whitespace(remaining);
        remaining = &remaining[whitespace_skipped..];

        let comment_skipped = skip_comment(remaining);
        remaining = &remaining[comment_skipped..];

        if whitespace_skipped + comment_skipped == 0 {
            return src.len() - remaining.len();
        }
    }
}

fn tokenize_identifier(input: &str) -> Result<(TokenKind, usize), Box<dyn Error>> {
    let identifier: String = input
        .chars()
        .take_while(|ch| *ch == '_' || ch.is_ascii_alphanumeric())
        .collect();

    let bytes_read = identifier.len();

    let result = if bytes_read == 0 {
        (TokenKind::Error("No identifier tokenized".to_string()), 0)
    } else if identifier.starts_with(|ch: char| ch != '_' && !ch.is_ascii_alphabetic()) {
        (TokenKind::Error("malformed identifier".to_string()), 0)
    } else {
        (TokenKind::Identifier(identifier), bytes_read)
    };

    Ok(result)
}

fn tokenize_number(input: &str) -> Result<(TokenKind, usize), Box<dyn Error>> {
    let mut number: String = input.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    let bytes_read = number.len();

    // did we stop at a decimal? If so, collect fractional number
    if let Some('.') = input.chars().nth(bytes_read) {
        let frac_number: String = input
            .chars()
            .skip(bytes_read + 1)
            .take_while(|ch| ch.is_ascii_digit())
            .collect();

        number.push('.');
        number.push_str(&frac_number);
    }

    let bytes_read = number.len();
    let number: f64 = number.parse()?;

    Ok((TokenKind::Number(number), bytes_read))
}

pub fn next_token(src: &str) -> Result<(TokenKind, usize), Box<dyn Error>> {
    let skipped_bytes = skip(src);
    let remaining = &src[skipped_bytes..];
    let mut iterator = remaining.chars();

    let next = match iterator.next() {
        Some(x) => x,
        None => return Ok((TokenKind::EndOfFile, skipped_bytes)),
    };

    let check_for_keyword =
        |keyword: &str, token: TokenKind| -> Result<(TokenKind, usize), Box<dyn Error>> {
            let k: String = remaining
                .chars()
                .take_while(|ch| ch.is_ascii_alphabetic())
                .collect();

            if k.eq(keyword) {
                Ok((token, keyword.len()))
            } else {
                tokenize_identifier(src)
            }
        };

    let mut relops_match = |true_token: TokenKind, false_token: TokenKind| -> (TokenKind, usize) {
        if let Some('=') = iterator.next() {
            (true_token, 2)
        } else {
            (false_token, 1)
        }
    };

    let (kind, length) = match next {
        '+' => (TokenKind::Plus, 1),
        '-' => (TokenKind::Minus, 1),
        ';' => (TokenKind::Semicolon, 1),
        ',' => (TokenKind::Comma, 1),
        '(' => (TokenKind::OpenParenthesis, 1),
        ')' => (TokenKind::CloseParenthesis, 1),

        // = or ==
        '=' => relops_match(TokenKind::EqualEqual, TokenKind::Equal),
        // ! or !=
        '!' => relops_match(TokenKind::BangEqual, TokenKind::Bang),
        // < or <=
        '<' => relops_match(TokenKind::LessEqual, TokenKind::Less),
        // > or >=
        '>' => relops_match(TokenKind::GreaterEqual, TokenKind::Greater),

        // check for keywords
        'c' => check_for_keyword("class", TokenKind::Class)?,
        'e' => check_for_keyword("else", TokenKind::Else)?,
        'i' => check_for_keyword("if", TokenKind::If)?,
        'n' => check_for_keyword("nil", TokenKind::Nil)?,
        'r' => check_for_keyword("return", TokenKind::Return)?,
        't' => check_for_keyword("true", TokenKind::True)?,
        'v' => check_for_keyword("var", TokenKind::Var)?,
        'w' => check_for_keyword("while", TokenKind::While)?,

        d @ '.' | d if d == '.' || d.is_ascii_digit() => tokenize_number(remaining)?,
        _ => tokenize_identifier(remaining)?,
    };

    Ok((kind, length + skipped_bytes))
}

#[cfg(test)]
pub mod lexer_tests {

    use super::*;

    #[test]
    fn relops_test() {
        // check the relops_match closure within next_token
        let expected_tokens = [
            TokenKind::Less,
            TokenKind::LessEqual,
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Equal,
            TokenKind::EqualEqual,
            TokenKind::Bang,
            TokenKind::BangEqual,
        ];

        let mut src = "< <= > >= = == ! !=";

        for curr_token in expected_tokens.iter() {
            let result = next_token(src);
            assert!(result.is_ok());
            let (token, bytes) = result.unwrap();

            // println!(
            //     "token: {:?}, curr_token: {:?}, bytes read: {}",
            //     token, *curr_token, bytes
            // );

            assert_eq!(token, *curr_token);
            src = &src[bytes..];
        }
    }

    #[test]
    fn skip_test() {
        let src = "              // this is a comment\n\n\n\nthis is not a comment";
        let index = skip(src);
        assert_eq!(index, 38);
        assert_eq!("this is not a comment", &src[index..]);

        let src = "";
        let index = skip(src);
        assert_eq!(0, index);

        let src = "// this is some comment\n            this is the text that is left";
        let index = skip(src);
        assert_eq!("this is the text that is left", &src[index..]);
    }

    #[test]
    fn skip_comment_test() {
        // only strip off the first newline.
        // save other newlines for whitespace stripping
        // Edge case #1
        let comment = "//\n\n\n\n";
        let skipped = skip_comment(comment);
        let comment = &comment[skipped..];
        assert_eq!(skipped, 3);
        assert_eq!("\n\n\n", comment);

        // Edge case #2
        let comment = "//";
        let skipped = skip_comment(comment);
        assert_eq!(skipped, 2);
        assert_eq!("", &comment[skipped..]);

        // Edge case #3
        // somehow non-comment string entered
        let comment = "this is not a comment at all";
        let skipped = skip_comment(comment);
        assert_eq!(skipped, 0);

        // no new line exists
        let comment = "// this is a comment without a newline at the end";
        let skipped = skip_comment(comment);
        assert_eq!(skipped, comment.len());

        // regular comment
        let comment = "// this is a comment\n\n\nthis is not a comment";
        let skipped = skip_comment(comment);
        assert_eq!(&comment[..skipped], "// this is a comment\n");
    }

    #[test]
    fn skip_whitespace_test() {
        let input = "  \n";
        let skipped = skip_whitespace(input);
        let input = &input[skipped..];
        assert_eq!(skipped, 3);
        assert_eq!(input, "");

        let input = "";
        let skipped = skip_whitespace(input);
        let input = &input[skipped..];
        assert_eq!(skipped, 0);
        assert_eq!(input, "");

        let input = "\t  \n  A+B=C";
        let skipped = skip_whitespace(input);
        let input = &input[skipped..];
        assert_eq!(input, "A+B=C");
        assert_eq!(skipped, 6);
    }

    #[test]
    fn tokenize_number_test() {
        let data = "45.55";
        let result = tokenize_number(data);
        assert!(result.is_ok());

        let data = "45";
        let result = tokenize_number(data);
        assert!(result.is_ok());

        let data = ".3456";
        let result = tokenize_number(data);
        assert!(result.is_ok());

        let data = "45 + 34";
        let result = tokenize_number(data);
        assert!(result.is_ok());

        let data = "";
        let result = tokenize_number(data);
        assert!(result.is_err());

        let data = "keith";
        let result = tokenize_number(data);
        assert!(result.is_err());
    }

    #[test]
    fn keyword_test() {
        let expected_tokens = [
            TokenKind::Class,
            TokenKind::Else,
            TokenKind::If,
            TokenKind::Nil,
            TokenKind::Return,
            TokenKind::True,
            TokenKind::Var,
            TokenKind::While,
        ];

        let mut src = "class else if nil return true var while";

        for token in expected_tokens.iter() {}

        for curr_token in expected_tokens.iter() {
            let result = next_token(src);
            assert!(result.is_ok());
            let (token, bytes) = result.unwrap();

            // println!(
            //     "token: {:?}, curr_token: {:?}, bytes read: {}",
            //     token, *curr_token, bytes
            // );

            assert_eq!(token, *curr_token);
            src = &src[bytes..];
        }
    }

    #[test]
    fn tokenize_identifier_test() {
        // degenerate case
        let src = "";
        let result = tokenize_identifier(src);
        assert!(result.is_ok());
        let (token, _) = result.unwrap();
        assert_eq!(
            TokenKind::Error("No identifier tokenized".to_string()),
            token
        );

        // test malformed identifier
        let src = "10ten";
        let result = tokenize_identifier(src);
        assert!(result.is_ok());
        let (token, _) = result.unwrap();
        assert_eq!(TokenKind::Error("malformed identifier".to_string()), token);

        // scans good part of identifier
        let src = "test@1234";
        let result = tokenize_identifier(src);
        assert!(result.is_ok());
    }

    #[test]
    fn next_token_test() {
        let src = "C=A+B;  // this is a comment\n_someValue = 30;";
        let remaining = src;

        let result = next_token(remaining);
        assert!(result.is_ok());
        let tuple = (TokenKind::Identifier("C".to_string()), 1);
        let remaining = &remaining[result.unwrap().1..];
        println!("remaining: {}", remaining);
        // assert_eq!(Ok(tuple), result);

        // tokenize the +
        // let result = next_token(input);
        // assert_eq!(Ok((Kind::Plus, 1)), result);

        // // tokenize the -
        // let input = &input[result.unwrap().1..];
        // let result = next_token(input);
        // assert_eq!(Ok((Kind::Minus, 1)), result);

        // // tokenize the =
        // let input = &input[result.unwrap().1..];
        // let result = next_token(input);
        // assert_eq!(Ok((Kind::Equals, 1)), result);

        // // tokenize the ;
        // let input = &input[result.unwrap().1..];
        // let result = next_token(input);
        // assert_eq!(Ok((Kind::Semicolon, 1)), result);

        // // tokenize identifier
        // let input = "      _someIdentifier";
        // let result = next_token(input);
        // assert!(result.is_ok());

        // // unexpected end of file
        // let input = "";
        // let result = next_token(input);
        // assert!(result.is_err());
    }
}
