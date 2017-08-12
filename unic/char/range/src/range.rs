use std::char;
use std::collections::Bound;
use CharIter;

/// A range of unicode code points.
///
/// The most idiomatic way to construct this range is through the use of the `chars!` macro:
///
/// ```
/// # #[macro_use] extern crate unic_char_range;
/// # use unic_char_range::*;
/// # fn main() {
/// assert_eq!(chars!('a'..='z'), CharRange::closed('a', 'z'));
/// assert_eq!(chars!('a'..'z'), CharRange::open_right('a', 'z'));
/// assert_eq!(chars!(..), CharRange::all());
/// # }
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CharRange {
    /// The lowest character in this range (inclusive).
    pub low: char,

    /// The highest character in this range (inclusive).
    pub high: char,
}

/// Constructors
impl CharRange {
    /// Construct a closed range of characters.
    ///
    /// If `stop` is ordered before `start`, the resulting range will be empty.
    pub fn closed(start: char, stop: char) -> CharRange {
        CharRange {
            low: start,
            high: stop,
        }
    }

    /// Construct a half open (right) range of characters.
    ///
    /// If `stop` is ordered before `start`, the resulting range will be empty.
    pub fn open_right(start: char, stop: char) -> CharRange {
        let mut iter = CharRange::closed(start, stop).iter();
        let _ = iter.next_back();
        iter.into()
    }

    /// Construct a half open (left) range of characters.
    ///
    /// If `stop` is ordered before `start`, the resulting range will be empty.
    pub fn open_left(start: char, stop: char) -> CharRange {
        let mut iter = CharRange::closed(start, stop).iter();
        let _ = iter.next();
        iter.into()
    }

    /// Construct a fully open range of characters.
    ///
    /// If `stop` is ordered before `start`, the resulting range will be empty.
    pub fn open(start: char, stop: char) -> CharRange {
        let mut iter = CharRange::closed(start, stop).iter();
        let _ = iter.next();
        let _ = iter.next_back();
        iter.into()
    }

    /// Construct a range of characters from bounds.
    ///
    /// If `stop` is ordered before `start`, the resulting range will be empty.
    pub fn bound(start: Bound<char>, stop: Bound<char>) -> CharRange {
        let start = if start == Bound::Unbounded {
            Bound::Included('\0')
        } else {
            start
        };
        let stop = if stop == Bound::Unbounded {
            Bound::Included(char::MAX)
        } else {
            stop
        };
        match (start, stop) {
            (Bound::Included(start), Bound::Included(stop)) => CharRange::closed(start, stop),
            (Bound::Excluded(start), Bound::Excluded(stop)) => CharRange::open(start, stop),
            (Bound::Included(start), Bound::Excluded(stop)) => CharRange::open_right(start, stop),
            (Bound::Excluded(start), Bound::Included(stop)) => CharRange::open_left(start, stop),
            (Bound::Unbounded, _) | (_, Bound::Unbounded) => unreachable!(),
        }
    }

    /// Construct a range over all characters.
    pub fn all() -> CharRange {
        CharRange::closed('\0', char::MAX)
    }
}

/// Collection-like fn
impl CharRange {
    /// Does this range include a character?
    ///
    /// # Examples
    ///
    /// ```
    /// # use unic_char_range::CharRange;
    /// assert!(   CharRange::closed('a', 'g').contains('d'));
    /// assert!( ! CharRange::closed('a', 'g').contains('z'));
    ///
    /// assert!( ! CharRange:: open ('a', 'a').contains('a'));
    /// assert!( ! CharRange::closed('z', 'a').contains('g'));
    /// ```
    pub fn contains(&self, ch: char) -> bool {
        self.low <= ch && ch <= self.high
    }

    /// How many characters are in this range?
    pub fn len(&self) -> usize {
        self.iter().len()
    }

    /// Create an iterator over this range.
    pub fn iter(&self) -> CharIter {
        (*self).into()
    }
}

impl IntoIterator for CharRange {
    type Item = char;
    type IntoIter = CharIter;

    fn into_iter(self) -> CharIter {
        self.iter()
    }
}
