pub fn revdigit(n: usize) -> i32 {
    let reversed_string: String = n.to_string().chars().rev().collect();
    println!("{} string ", reversed_string);
    return reversed_string.parse::<i32>();
}
