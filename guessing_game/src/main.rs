use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();

    println!("Guess the number!!");

    println!("Please input your guess.");

    let readline = rl.readline(">> ");

    let guess = readline.expect("Failed to read line");

    println!("You guessed: {}", guess);
}
