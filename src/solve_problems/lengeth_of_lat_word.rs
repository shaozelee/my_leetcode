pub fn length_of_last_word(s: String) -> i32 {
    let vec: Vec<&str> = s.split(' ').collect();
    let last_index = vec.len() - 1;
    vec[last_index].len() as i32
}
