fn borrow(s: &[i32]) {
    println!("borrowed = {:?}", s);
}

fn borrow_mut(s: &mut [i32]) {
    s[0] = 100;
}

fn main() {
    let a: [i32;5] = [1,2,3,4,5];
    let s: &[i32] = &a[0..2]; // Slice is a reference
    borrow(s);
    println!("s = {:?}", s);

    let mut a: [i32;5] = [1,2,3,4,5];
    let s: &mut [i32] = &mut a[0..2]; // Slice is a mutable reference
    borrow_mut(s);
    println!("after mutation s = {:?}", s);
}