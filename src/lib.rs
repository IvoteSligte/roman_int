use std::fmt::Display;

use num_traits::AsPrimitive;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Wrapper around a string, which is the converted numeral
///
/// Integer input range is 1..=3999
///
/// Use [From<T>](https://doc.rust-lang.org/std/convert/trait.From.html)
/// and [Into<T>](https://doc.rust-lang.org/std/convert/trait.Into.html)
/// to convert from and to an integer type.
pub struct Numeral(String);

impl Display for Numeral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: AsPrimitive<usize>> From<T> for Numeral {
    fn from(num: T) -> Self {
        let mut n: usize = num.as_();

        assert!(n < 4000);

        let mut string = String::new();

        fn digit(n: usize, one: char, five: char, ten: char) -> String {
            match n {
                0 => String::new(),
                1 => format!("{0}", one),
                2 => format!("{0}{0}", one),
                3 => format!("{0}{0}{0}", one),
                4 => format!("{0}{1}", one, five),
                5 => format!("{0}", five),
                6 => format!("{1}{0}", one, five),
                7 => format!("{1}{0}{0}", one, five),
                8 => format!("{1}{0}{0}{0}", one, five),
                9 => format!("{0}{1}", one, ten),
                _ => panic!(),
            }
        }

        string = digit(n % 10, 'I', 'V', 'X') + &string;
        n /= 10;

        string = digit(n % 10, 'X', 'L', 'C') + &string;
        n /= 10;

        string = digit(n % 10, 'C', 'D', 'M') + &string;
        n /= 10;

        string = match n % 10 {
            0 => String::new(),
            1 => format!("{0}", 'M'),
            2 => format!("{0}{0}", 'M'),
            3 => format!("{0}{0}{0}", 'M'),
            _ => panic!(),
        } + &string;

        Self(string)
    }
}
