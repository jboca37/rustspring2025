fn clone_and_modify(s: &String) -> String {
    // Your code here
    let mut modified = s.clone();
    modified.push_str("World!");
    modified
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}