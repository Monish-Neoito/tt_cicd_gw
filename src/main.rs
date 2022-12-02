use std::io;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let mut _num_str_1 = String::new();
    let mut _num_str_2 = String::new();
    io::stdin().read_line(&mut _num_str_1).expect("read error");
    io::stdin().read_line(&mut _num_str_2).expect("read error");
    let mut _num_1 : i32 = _num_str_1.trim().parse().expect("parse error");
    let mut _num_2 : i32 = _num_str_2.trim().parse().expect("parse error");
    println!("{}", add(_num_1, _num_2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}