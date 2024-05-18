fn main() {
    println!("Hello, world!");
}
#[allow(dead_code)]
fn answer() -> u32 {
    42
}

pub fn not_tested() -> u32 {
    42
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn deep_thought_test(){
        assert_eq!(answer(),42);

    }
}