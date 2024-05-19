
fn main() {
    println!("Hello, world!");
    
}


#[allow(dead_code)]
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
    fn num_test_greater_success() {
        assert_eq!(greater_than_5(6), 1);
    }

    #[test]
    fn num_test_greater_failed() {
        assert_eq!(greater_than_5(4), 0);
    }

    #[test]
    fn test_main() {
        assert_eq!(main(), ());
    }
}

// cargo tarpaulin --ignore-tests --target-dir target/tarpaulin-target/ --skip-clean --out Lcov

// all test
// cargo tarpaulin --ignore-tests --target-dir target/tarpaulin-target/ --skip-clean --out Lcov  -- --nocapture

// cargo tarpaulin --target-dir target/tarpaulin-target/ --skip-clean --out Lcov