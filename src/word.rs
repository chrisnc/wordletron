use custom_derive::custom_derive;
use newtype_derive::*;

use std::array::TryFromSliceError;
use std::fmt;
use std::iter;

use super::N;

custom_derive! {
    #[derive(NewtypeFrom, NewtypeIndex(usize), NewtypeIndexMut(usize), PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
    pub struct Word(pub [u8; N]);
}

impl Word {
    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<u8> {
        self.0.iter_mut()
    }
}

impl TryFrom<&[u8]> for Word {
    type Error = TryFromSliceError;

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Ok(Word(s.try_into()?))
    }
}

impl TryFrom<&str> for Word {
    type Error = TryFromSliceError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Word(s.as_bytes().try_into()?))
    }
}

impl From<&Word> for String {
    fn from(w: &Word) -> Self {
        String::from_utf8_lossy(&w.0).to_string()
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}

impl iter::IntoIterator for Word {
    type Item = u8;
    type IntoIter = std::array::IntoIter<Self::Item, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
