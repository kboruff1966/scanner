fn main() {
    // let src = "A=B+C;   // this is a comment   \nSomeIdentifier\n";
    // let mut remaining = src;

    // while remaining.len() != 0 {
    //     let result = next_token(remaining);
    //     println!("{:?}", result);
    //     if let Ok(x) = result {
    //         remaining = &remaining[(x.1)..];
    //         println!("`{}`", remaining);
    //     }
    // }

    let x = 0123;
    println!("{}", x);
}

// TOKEN_AND, TOKEN_CLASS, TOKEN_ELSE, TOKEN_FALSE,
//   TOKEN_FOR, TOKEN_FUN, TOKEN_IF, TOKEN_NIL, TOKEN_OR,
//   TOKEN_PRINT, TOKEN_RETURN, TOKEN_SUPER, TOKEN_THIS,
//   TOKEN_TRUE, TOKEN_VAR, TOKEN_WHILE,

use std::error::Error;

#[derive(Debug, PartialEq)]
enum Kind {
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

    Number(f64),
    Identifier(String),
    Equals,
    Plus,
    Minus,
    Semicolon,
    EndOfFile,
    Error(String),
}

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
        if let Some(x) = src.find('\n') {
            x + 1
        } else {
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

fn check_for_keyword(
    src: &str,
    keyword: &str,
    token: Kind,
) -> Result<(Kind, usize), Box<dyn Error>> {
    let k: String = src
        .chars()
        .take_while(|ch| ch.is_ascii_alphabetic())
        .collect();

    if k.eq(keyword) {
        Ok((token, keyword.len()))
    } else {
        tokenize_identifier(src)
    }
}

fn tokenize_identifier(input: &str) -> Result<(Kind, usize), Box<dyn Error>> {
    let identifier: String = input
        .chars()
        .take_while(|ch| *ch == '_' || ch.is_ascii_alphanumeric())
        .collect();

    let bytes_read = identifier.len();

    let result = if bytes_read == 0 {
        (Kind::Error("No identifier tokenized".to_string()), 0)
    } else if identifier.starts_with(|ch: char| ch != '_' && !ch.is_ascii_alphabetic()) {
        (Kind::Error("malformed identifier".to_string()), 0)
    } else {
        (Kind::Identifier(identifier), bytes_read)
    };

    Ok(result)
}

fn tokenize_number(input: &str) -> Result<(Kind, usize), Box<dyn Error>> {
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

    Ok((Kind::Number(number), bytes_read))
}

fn next_token(src: &str) -> Result<(Kind, usize), Box<dyn Error>> {
    let cursor = skip(src);
    let remaining = &src[cursor..];

    let next = match remaining.chars().next() {
        Some(x) => x,
        None => return Ok((Kind::EndOfFile, cursor)),
    };

    let (kind, length) = match next {
        '=' => (Kind::Equals, 1),
        '+' => (Kind::Plus, 1),
        '-' => (Kind::Minus, 1),
        ';' => (Kind::Semicolon, 1),

        // check for keywords
        'c' => check_for_keyword(remaining, "class", Kind::Class)?,
        'e' => check_for_keyword(remaining, "else", Kind::Else)?,
        'i' => check_for_keyword(remaining, "if", Kind::If)?,
        'n' => check_for_keyword(remaining, "nil", Kind::Nil)?,
        'r' => check_for_keyword(remaining, "return", Kind::Return)?,
        't' => check_for_keyword(remaining, "true", Kind::True)?,
        'v' => check_for_keyword(remaining, "var", Kind::Var)?,
        'w' => check_for_keyword(remaining, "while", Kind::While)?,

        d @ '.' | d if d == '.' || d.is_ascii_digit() => tokenize_number(remaining)?,
        _ => tokenize_identifier(remaining)?,
    };

    Ok((kind, length + cursor))
}

#[cfg(test)]
mod tests {

    use super::*;

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
    fn tokenize_identifier_test() {
        // degenerate case
        let src = "";
        let result = tokenize_identifier(src);
        assert!(result.is_ok());
        let (token, _) = result.unwrap();
        assert_eq!(Kind::Error("No identifier tokenized".to_string()), token);

        // test malformed identifier
        let src = "10ten";
        let result = tokenize_identifier(src);
        assert!(result.is_ok());
        let (token, _) = result.unwrap();
        assert_eq!(Kind::Error("malformed identifier".to_string()), token);

        // scans good part of identifier
        let src = "test@1234";
        let result = tokenize_identifier(src);
        assert!(result.is_ok());
    }

    // #[test]
    fn next_token_test() {
        let src = "C=A+B;  // this is a comment\n_someValue = 30;";
        let remaining = src;

        let result = next_token(remaining);
        assert!(result.is_ok());
        let tuple = (Kind::Identifier("C".to_string()), 1);
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
