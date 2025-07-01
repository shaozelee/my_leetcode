// 输入: numRows = 5
// 输出: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
/*
            1
           1 1
          1 2 1
         1 3 3 1
        1 4 6 5 1
*/

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let num_rows = num_rows as usize;
    let mut output_vec: Vec<Vec<i32>> = Vec::with_capacity(num_rows);
    for i in 0..num_rows {
        let mut row = vec![1; i + 1];
        for j in 1..i {
            row[j] = output_vec[i - 1][j - 1] + output_vec[i - 1][j];
        }
        output_vec.push(row);
    }
    output_vec
}

#[test]
fn test_generate() {
    let num_rows = 4;
    println!("{:?}", generate(num_rows));
}
