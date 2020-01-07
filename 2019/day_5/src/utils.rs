pub fn number_to_vec(n: i32) -> (Vec<i32>, i32) {
    let mut digits= Vec::new();
    let mut n = n;
    let mut i = 0;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
        i += 1;
    }
    digits.push(n);

    (digits, i)
}

//pub fn right_pad_with_zeros(mut vector: Vec<i32>, size: i32) -> Vec<i32> {
//    for i in 0..size {
//        vector.push(0)
//    }
//    vector
//}

pub fn left_pad_with_zeros(mut vector: Vec<i32>, size: i32) -> Vec<i32> {
    for _i in 0..size {
        vector.push(0)
    }
    vector.reverse();
    vector
}