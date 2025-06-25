mod solve_problems;

fn main() {
    let result = solve_problems::two_sum::two_sum(vec![1, 2, 4, 5], 5);
    println!("{:?}", result);

    let num = 989;
    let result = solve_problems::is_palindrom::is_palindrome(num);
    println!("{}", result);

    let result = solve_problems::roman_to_num::roman_to_int("MIVVVVI".to_string());
    println!("{}", result);
}
