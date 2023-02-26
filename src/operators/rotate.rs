pub fn rotate_to_edge(data: &mut [i32], left: i32, right: i32) {
    if left == data[data.len() - 1] && right == data[0] {
        data.reverse();
    } else {
        let idx_left = data.iter().position(|&x| x == left).unwrap();
        let idx_right = data.iter().position(|&x| x == right).unwrap();
        if idx_left > idx_right {
            data.rotate_left(idx_left);
        } else {
            data.rotate_left(idx_right);
            data.reverse()
        }
    }
}