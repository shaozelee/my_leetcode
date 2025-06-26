pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result = String::new();
    //  str is : "lsssa" ,"lsssb" , "lsssc"
    for (i, val) in strs.iter().enumerate() {
        if i == 0 {
            result = val.to_string();
        } else {
            // let mut j = 0;
            // while j < result.len() && j < val.len() && result.chars().nth(j) == val.chars().nth(j) {
            //     j += 1;
            // }
            // result = result[0..j].to_string();
            let mut j = 0;
            let result_chars: Vec<char> = result.chars().collect();
            let val_chars: Vec<char> = val.chars().collect();
            while j < result_chars.len() && j < val_chars.len() && result_chars[j] == val_chars[j] {
                j += 1;
            }
            result = result_chars[..j].iter().collect();
        }
    }

    return result;
}
