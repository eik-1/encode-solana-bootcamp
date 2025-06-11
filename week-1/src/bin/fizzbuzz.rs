fn main() {
    println!("Welcome to FizzBuzz");
    let count:usize = fizzbuzz();
    println!("The count is {}", count);

}

fn fizzbuzz() -> usize {
    let mut count:usize = 0;
    for i in 1..=301 {
        match(i % 3, i % 5) {
            (0,0) => {
                println!("FizzBuzz");
                count+=1;
            },
            (0,_) => println!("Fizz"),
            (_,0) => println!("Buzz"),
            (_,_) => print!(""),
        }
    }
    count
}