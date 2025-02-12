fn is_even(n: i32) -> bool {
    match n % 2 {
        0 => true,
        _ => false,
    }
}

fn main() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for n in array {
        let is_even = is_even(n);
        match is_even {
            true => println!("{n} is even"),
            false => println!("{n} is odd"),
        }

        let by_3 = n % 3 == 0;
        let by_5 = n % 5 == 0;

        match (by_3, by_5) {
            (true, true) => println!("{n} is divisible by 3 & 5:FizzBuzz"),
            (true, false) => println!("{n} is divisible by 3: Fizz"),
            (false, true) => println!("{n} is divisible by 5: Buzz"),
            (false, false) => println!("{n} is not divisible by 3 or 5"),
        }
    }

    let mut sum_of_all_array_elements = 0;
    let mut counter = 0;

    while counter < array.len() {
        counter += 1;

        sum_of_all_array_elements += array[counter - 1];
    }

    println!("Sum of all array elements is: {sum_of_all_array_elements}");

    let mut max = array[0];

    for n in array {
        if n > max {
            max = n;
        }
    }

    println!("Max number in our array is: {max}");
}
