fn main() {
    let v: [i32;5] = [0,1,2,3,4];
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();

    println!("The doubled array is: {:?}", doubled);

}