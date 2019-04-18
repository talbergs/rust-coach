pub mod arger {
    use std::env;

    pub fn to_vec() -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        for i in env::args().skip(1) {
            v.push(i.parse().unwrap());
        }
        v
    }
}

pub mod printer {
    pub fn comment(i: i32) {
        println!("# => {}", i);
    }
}
