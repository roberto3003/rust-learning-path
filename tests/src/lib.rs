#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2));
            // assert_eq!(5, add_two(2));  Will fail
            // assert_ne!(4, add_two(2)); Will fail because assertion is: 4 != 4;
        }
    }

    pub fn greeting(name: &str) -> String {
        // format!("Hello {}!", name)
        String::from("Hello") // Bug introduced
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    pub struct Guess {
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Guess {
            // if value < 1 || value > 100 {
            // Add a bug:
            if value < 1 {
                // panic!("Guess value must be between 1 and 100, got {}.", value);
                // Introduced a bug by swapping the bodies of if and else if:
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            } else if value > 100 {
                // panic!("Guess value must be less than or equal to 100, got {}.", value);
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    pub fn add_three(a: i32) -> i32 {
        internal_adder(a, 2)
    }

    fn internal_adder(a: i32, b: i32) -> i32 {
        a + b
    }
}
