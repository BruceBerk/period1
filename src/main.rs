use std::io::stdin;

fn load_txns() {
    println!("Loading period txns list");
}

fn process_txns() {
    println!("Processing period item list");

    let mut user_ans = String::new();
    println!("What do you want to do? (a)dd, (s)kip or (i)gnore?");
    stdin()
        .read_line(&mut user_ans)
        .expect("Failed to read line");
}

fn write_txns() {
    println!("Writing revised period items list");
}

fn main() {
    load_txns();

    process_txns();

    write_txns();
}