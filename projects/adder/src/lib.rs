#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

pub fn greet(name: &str) -> String {
    format!("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than 1");
        } else if value > 100 {
            panic!("Guess value must be less than 100");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_contains_name() {
        let result = greet("Andrew");
        assert!(
            result.contains("Andrew"),
            "Greeting did not contain name, value was `{result}`");
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_adds_two_result() -> Result<(), String> {
        let result = add_two(2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("Two plust two did not equal four"))
        }
    }
}
