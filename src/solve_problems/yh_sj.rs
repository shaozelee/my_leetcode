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

// 输入: rowIndex = 3
// 输出: [1,3,3,1]
pub fn get_row(row_index: i32) -> Vec<i32> {
    let row_index = row_index as usize;
    let mut row = vec![1];
    for i in 1..=row_index {
        // 从后往前更新，避免覆盖还未用到的值
        row.push(1);
        for j in (1..i).rev() {
            row[j] += row[j - 1];
        }
    }
    row
}
#[test]
fn test_generate() {
    let num_rows = 4;
    println!("{:?}", generate(num_rows));
}

#[test]
fn test_get_row() {
    let row_index = 3;
    let result = get_row(row_index);
    println!("{:?}", result);
}
