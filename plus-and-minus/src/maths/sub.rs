pub fn from_vec(digs: Vec<i32>) -> i32 {
    let mut digsi = digs.iter();
    let dig = digsi.next().unwrap();
    digsi.fold(*dig, |acc, i| acc - i)
}

