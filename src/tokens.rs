enum Token {
    Plus,
    Mult,
    Number,
    CurlyOpen,
    CurlyClose,
    End,
    Inval
}
fn run() {
    println!("{}",number("12"));
}

fn number(s: &str) -> i32 {
    return s as i32;
}