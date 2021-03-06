#[derive(Debug)]
struct Rectangle {
    width: u32,
    high: u32,
}
impl Rectangle {
    fn can_hold (&self, other: &Rectangle) -> bool {
        self.width>other.width && self.high>other.high
    }
}

pub fn add_two (a:i32) -> i32 {
    a+ 2
}
//this is for the test to make the assert! panic
// pub fn greeting (name: &str) -> String {
//     String::from("Hello !")
// }

// this the normal one
pub fn greeting (name: &str) -> String {
    format!("Hello ! {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn larger_can_hold_smaller () {
        let larger = Rectangle {
            width: 8,
            high: 7,
        };
        let smaller = Rectangle {
            width: 5,
            high: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger () {
        let larger = Rectangle{
            width:8,
            high:7,
        };
        let smaller = Rectangle {
            width:5,
            high:1,
        };
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn it_adds_two () {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn greeting_name () {
        let result = greeting("carlo");
        assert!(result.contains("carlo"),
                "Greetings did contain name , value was {}",
                result);
    }


}