use std::io;
use std::io::Write;

fn main() {
    print!("\x1B[?1049h"); //enter alternate screen
    print!("\x1B[H");
    print!("alternate screen mode");
    print!("\x1B\x5B9B"); //move cursor 9 lines down

    // TODO Exit alternate screen to restore terminal

    io::stdout().flush();

    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);
    println!("{:?}", result);
}
