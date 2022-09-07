fn count_to_5() -> i32 {
    let mut result = 0;
    for _ in 0..5 {
        result += 1;
    }
    result
}

fn main() {
    println!("I can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}
