use std::fmt;

pub enum Part {
    One,
    Two,
}

pub trait GetInput {
    type Input;

    fn text_to_input(content: &str) -> Self::Input;

    fn data() -> Self::Input;

    fn example() -> Self::Input;
}

/// Aoc Challenge interface
pub trait Solve: GetInput {
    fn part1() -> Output {
        "`Part 1 is not yet implemented`".into()
    }

    fn part2() -> Output {
        "`Part 2 is not yet implemented`".into()
    }

    fn solve(part: Part) -> Output {
        match part {
            Part::One => Self::part1(),
            Part::Two => Self::part2(),
        }
    }
}

// @ericwburden
macro_rules! impl_output_from {
    ($([$e:tt, $t:ty]),*) => {
        #[derive(Debug, Eq)]
        pub enum Output { $( $e($t), )* }
        $(impl From<$t> for Output {
            fn from(value: $t) -> Self {
                Output::$e(value)
            }
        })*
    };
}

impl_output_from! {
    [ U8,     u8 ],
    [ U16,    u16 ],
    [ U32,    u32 ],
    [ U64,    u64 ],
    [ U128,   u128 ],
    [ I8,     i8 ],
    [ I16,    i16 ],
    [ I32,    i32 ],
    [ I64,    i64 ],
    [ I128,   i128 ],
    [ String, String ]
}

impl From<&str> for Output {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Output::U8(v) => write!(f, "{v}"),
            Output::U16(v) => write!(f, "{v}"),
            Output::U32(v) => write!(f, "{v}"),
            Output::U64(v) => write!(f, "{v}"),
            Output::U128(v) => write!(f, "{v}"),
            Output::I8(v) => write!(f, "{v}"),
            Output::I16(v) => write!(f, "{v}"),
            Output::I32(v) => write!(f, "{v}"),
            Output::I64(v) => write!(f, "{v}"),
            Output::I128(v) => write!(f, "{v}"),
            Output::String(v) => write!(f, "{v}"),
        }
    }
}

impl<T: fmt::Display> PartialEq<T> for Output {
    fn eq(&self, other: &T) -> bool {
        *self.to_string() == other.to_string()
    }
}
