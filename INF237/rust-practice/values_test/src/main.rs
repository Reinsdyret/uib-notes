fn main() {
    let mut test_vec: Vec<u8> = vec![65, 66, 67];
    println!("{:?}", &test_vec.clone().into_iter().map(|c| c as char).collect::<Vec<char>>());

    test(&mut test_vec);

    println!("{:?}", test_vec.into_iter().map(|c| c as char).collect::<Vec<char>>());
}

fn test(vec: &mut Vec<u8>) {
    vec[1] += 1;
}
