fn main() {
    let mut s1: String = String::from("Hello World!");
    change(&mut s1);
    let len: usize = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(" Bootcamp"); // Error since this isn't a mutable referant
    let length: usize = s.len();
    length
}

fn change(s: &mut String) {
    s.push_str(" Bootcamp");
    println!("New string is {}", s);
}