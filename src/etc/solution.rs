use std::fmt::{Display, Formatter, Result};
use Solution::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Solution {
    U64(u64),
    Usize(usize),
    Str(String),
    Todo(),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            U64(x) => x.fmt(f),
            Usize(x) => x.fmt(f),
            Str(x) => x.fmt(f),
            Todo() => "TODO".fmt(f),
        }
    }
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    };
}

impl_from!(u64, U64);
impl_from!(usize, Usize);
impl_from!(String, Str);

impl From<&str> for Solution {
    fn from(sol: &str) -> Self {
        Self::Str(sol.to_owned())
    }
}

impl From<()> for Solution {
    fn from(_: ()) -> Self {
        Self::Todo()
    }
}
