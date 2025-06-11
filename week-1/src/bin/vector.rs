fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    println!("v[1] = {}", v[1]);

    match v.get(100) {
        Some(val) => println!("v[100] = {}", val),
        None => println!("no such element"),
    }

    for val in &v {
        println!("{}", val);
    }
}