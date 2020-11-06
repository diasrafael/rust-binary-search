// o(log n) time | o(log n) space

#[allow(dead_code)]
pub fn binary_search(array:Vec<i32>, target: i32) -> i32 {
    if array.len() == 1 {
        if array[0] == target { 
            return 0
        } else {
            return -1
        }
    } else {
        let middle =  array.len() / 2;
        let left = &array[0..middle];
        let right = &array[middle..array.len()];

        let left_index = binary_search(left.to_vec(), target);
        let right_index = binary_search(right.to_vec(), target);

        if left_index > -1 {
            return left_index    
        }

        if right_index > -1 {
            return (left.len() + (right_index as usize)) as i32 
        }
    }
    return -1
}

