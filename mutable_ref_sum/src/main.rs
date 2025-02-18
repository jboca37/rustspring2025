#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    for i in low..=high {
        *total += i;
    }
}

fn main() {
    // create necessary variables and test your function for low 0 high 100
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Sum from 0 to 100 is: {total}"); // total should be 5050
}