fn main() {
    println!("Hello, world!");
    println!("{}{}", answer(), not_tested());
}
// #[allow(dead_code)]
fn answer() -> u32 {
    42
}

// #[allow(dead_code)]
fn not_tested() -> u32 {
    42
}

fn greater_than_5(num: u32) -> u32 {
    if num > 5 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn deep_thought_test() {
        assert_eq!(answer(), 42);
    }

    #[test]
    fn test_not_tested() {
        assert_eq!(not_tested(), 42);
    }

    #[test]
    fn num_test(){
        assert_eq!(greater_than_5(6),1);
    }
}

// cargo tarpaulin --ignore-tests --target-dir target/tarpaulin-target/ --skip-clean --out Lcov