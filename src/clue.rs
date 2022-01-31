use custom_derive::custom_derive;
use newtype_derive::*;

use std::array::TryFromSliceError;
use std::fmt;
use std::iter;

use super::N;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum ClueColor {
    Green,
    Yellow,
    Black,
}
pub use ClueColor::*;

impl From<&ClueColor> for &'static str {
    fn from(c: &ClueColor) -> Self {
        match c {
            Green => "🟩",
            Yellow => "🟨",
            Black => "⬛️",
        }
    }
}

custom_derive! {
    #[derive(NewtypeFrom, NewtypeIndex(usize), NewtypeIndexMut(usize), PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
    pub struct Clue(pub [ClueColor; N]);
}

impl Clue {
    pub fn iter(&self) -> std::slice::Iter<ClueColor> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<ClueColor> {
        self.0.iter_mut()
    }
}

impl From<&Clue> for String {
    fn from(clue: &Clue) -> Self {
        clue.0.iter().map(<&str>::from).collect()
    }
}

impl TryFrom<char> for ClueColor {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_lowercase().next() {
            Some('g') => Ok(Green),
            Some('y') => Ok(Yellow),
            Some('b') => Ok(Black),
            _ => Err("invalid color"),
        }
    }
}

impl TryFrom<&[ClueColor]> for Clue {
    type Error = TryFromSliceError;

    fn try_from(s: &[ClueColor]) -> Result<Self, Self::Error> {
        Ok(Clue(s.try_into()?))
    }
}

impl fmt::Display for Clue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}

impl iter::IntoIterator for Clue {
    type Item = ClueColor;
    type IntoIter = std::array::IntoIter<Self::Item, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
