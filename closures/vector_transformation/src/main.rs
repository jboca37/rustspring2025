// Implementation using map and collect
fn process_vector_with_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

// Implementation using a for loop
fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x)); // Apply the closure
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3];

    // Using map and collect
    let doubled_map = process_vector_with_map(numbers.clone(), |x| x * 2);
    let replaced_map = process_vector_with_map(numbers.clone(), |x| if x > 2 { 0 } else { x });
    println!("Doubled (map): {:?}", doubled_map);
    println!("Replaced (map): {:?}", replaced_map);

    // Using a for loop
    let doubled_for_loop = process_vector_with_for_loop(numbers.clone(), |x| x * 2);
    let replaced_for_loop = process_vector_with_for_loop(numbers, |x| if x > 2 { 0 } else { x });
    println!("Doubled (for loop): {:?}", doubled_for_loop);
    println!("Replaced (for loop): {:?}", replaced_for_loop);
}