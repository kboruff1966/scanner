fn main() {
    let src = "A=B+C;   // this is a comment   \nSomeIdentifier\n";
    let mut remaining = src;

    while remaining.len() != 0 {
        let result = next_token(remaining);
        println!("{:?}", result);
        if let Ok(x) = result {
            remaining = &remaining[(x.1)..];
            println!("`{}`", remaining);
        }
    }
}

#[derive(Debug, PartialEq)]
enum Kind {
    Integer(i32),
    // Float(f64),
    Identifier(String),
    Equals,
    Plus,
    Minus,
    Semicolon,
    EndOfFile,
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
    if !src.starts_with("//") {
        return 0;
    }

    let bytes = match src.char_indices().take_while(|ch| ch.1 != '\n').last() {
        Some(x) => x.0 + 1,
        None => 0,
    };

    // scan past closing newline if exists
    if let Some('\n') = &src[bytes..].chars().next() {
        bytes + 1
    } else {
        bytes
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

fn tokenize_identifier(input: &str) -> Result<(Kind, usize), String> {
    match input.chars().next() {
        Some(c) if c != '_' && !c.is_ascii_alphabetic() => {
            return Err("malformed identifier".to_string())
        }
        None => return Ok((Kind::EndOfFile, 0)),
        _ => (),
    }

    let identifier: String = input
        .chars()
        .take_while(|ch| *ch == '_' || ch.is_ascii_alphanumeric())
        .collect();

    let bytes_read = identifier.len();

    Ok((Kind::Identifier(identifier), bytes_read))
}

fn next_token(src: &str) -> Result<(Kind, usize), String> {
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
        c @ '_' | c if c == '_' || c.is_ascii_alphabetic() => tokenize_identifier(remaining)?,
        '0'..='9' => (Kind::Integer(45), 1),
        other => return Err(format!("Unknown character '{}'", other)),
    };

    Ok((kind, length + cursor))
}

// fn tokenize_number(input: &str) -> Result<(Kind, usize), String> {
//     Ok((Kind::Integer(45), 2))
// }

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

    // #[test]
    // fn tokenize_number_test() {
    //     let data = "45";
    //     let result = tokenize_number(data);
    // }

    #[test]
    fn tokenize_identifier_test() {
        // degenerate case
        let src = "";
        let result = tokenize_identifier(src);
        assert_eq!(Ok((Kind::EndOfFile, 0)), result);

        // test malformed identifier
        let src = "10ten";
        let result = tokenize_identifier(src);
        assert!(result.is_err());

        // another malformed case
        let src = "     someID";
        let result = tokenize_identifier(src);
        assert!(result.is_err());

        // scans good part of identifier
        let src = "test@1234";
        let result = tokenize_identifier(src);
        assert_eq!(Ok((Kind::Identifier("test".to_string()), 4)), result);
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
