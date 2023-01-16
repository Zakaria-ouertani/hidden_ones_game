// fn matrix_max_len()
pub fn array_max_len(array: &Vec<String>) -> usize {
    let mut max_val: usize = array[0].len();
    for i in array {
        if i.len() > max_val {
            max_val = i.len()
        }
    }
    return max_val;
}

pub fn matrix_max_len(matrix: &Vec<Vec<String>>) -> usize {
    let mut max_val: usize = array_max_len(&matrix[0]);
    let mut next_val: usize;
    for i in matrix {
        next_val = array_max_len(i);
        if next_val > max_val {
            max_val = next_val
        }
    }
    return max_val;
}