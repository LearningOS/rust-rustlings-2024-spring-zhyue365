// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(data.clone());

    println!("The last character was '{}'", last_char);

    let u_data = string_uppercase(data);

    println!("{}", u_data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership and return a new String
fn string_uppercase(data: String) -> String {
    data.to_uppercase()
}