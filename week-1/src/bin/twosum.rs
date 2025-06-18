fn two_sum(nums: &Vec<i32>, target: &i32) -> Vec<i32> {

    for (i, num1) in nums.iter().enumerate() {
        for (j, num2) in nums.iter().skip(i+1).enumerate() {
            if num1 + num2 == *target {
                return vec![i as i32, (j + i + 1) as i32];
            }
        }
    }
    vec![]
}

fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 18;
    let result: Vec<i32> = two_sum(&nums, &target);
    println!("{:?}", result);
}