mod lexer;

fn main() {
    let src = "A=B+C;   // this is a comment   \nSomeIdentifier\n";
    let mut remaining = src;

    while remaining.len() != 0 {
        let result = lexer::next_token(remaining);
        println!("{:?}", result);
        if let Ok(x) = result {
            remaining = &remaining[(x.1)..];
            println!("`{}`", remaining);
        }
    }

    let x = 0123;
    println!("{}", x);

    println!("lets do another modification");
}
