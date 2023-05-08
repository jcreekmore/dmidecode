//! Handle *Bit Field* values in SMBIOS structure
//!
//! Due to SMBIOS information has read-only nature and *Bit Field* flags has text description it is
//! reasonable to have special type(s) to handle them

use core::fmt;
use core::convert::TryFrom;
use core::iter::FromIterator;
use core::ops::{Deref, RangeInclusive};


/// Handles single flag within *Bit Field*
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Flag<'a> {
    pub position: Position,
    pub is_set: bool,
    pub type_: FlagType<'a>,
}

/// There are 2 types of *Bit Field* flag meaningful and reserved for some purposes
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq,)]
pub enum FlagType<'a> {
    Unknown,
    Significant(&'a str, &'a str),
    Reserved(&'a str),
}

/// Alias to Bit Field description "table"
pub type Layout<'a> = &'a [FlagType<'a>];

/// An iterator through **all** items in *Bit Field* layout
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Default,)]
pub struct Iter<'a, T> {
    pub value: T,
    pub index: usize,
    pub layout: Layout<'a>
}

/// Flag position (index) in *Bit Field*
#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct Position(pub usize);

/// An iterator through **setted** non-reserved flags
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Default,)]
pub struct Significants<'a, T>(Iter<'a, T>);

/// An iterator returns ranges of reserved bits folded from [Iter]
///
/// There are may be multiple types of reserved flags groupped by reserved type description
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct Reserved<'a, T> {
    iter: Iter<'a, T>,
    desc: Option<&'a str>,
    start: usize,
}

/// Reserved bits range
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ReservedRange<'a> {
    pub description: &'a str,
    pub range: RangeInclusive<usize>
}


/// Trait for Bit Field data
pub trait BitField<'a> {
    type Size: Default + Into<u128> + TryFrom<u128> + Copy + fmt::Debug;
    const LAYOUT: Layout<'a> = &[];
    fn value(&self) -> Self::Size;
    fn iter(&self) -> Iter<'a, Self::Size> {
        Iter::new(self.value(), Self::LAYOUT)
    }
    fn significants(&self) -> Significants<'a, Self::Size> {
        Significants::new(self.iter())
    }
    fn reserved(&self) -> Reserved<'a, Self::Size> {
        Reserved::new(self.iter())
    }
}


/// Convenient way to create [BitField::LAYOUT] slice
///
/// Short form refers to [https://www.nongnu.org/dmidecode/] source code
/// Long form refers to SMBIOS Specification
/// Assume internal use only
///
/// ```ignore
/// # #[macro_use]
/// # extern crate dmidecode;
/// # use dmidecode::bitfield::{FlagType, Layout};
/// # fn main() {
/// layout!(
///     // Certain slice length
///     length = 4;
///     // Short form of [FlagType::Significant]: one text duplicated to meaning and description
///     "A",
///     // Long form of [FlagType::Significant]: first string is description, second is long description
///     "B" "B Long",
///     // [FlagType::Reserved] fields defines as "Text": Length
///     "Reserved": 2,
/// );
/// # }
/// ```
/// It is mandatory to have trailing comma on last item!
macro_rules! layout {
    // Initial call
    (length = $len:expr; $($tail:tt)*) => {
        const LAYOUT: Layout<'static> = &layout!(array = [FlagType::Unknown; $len], index = 0; $($tail)*);
    };
    // Terminating scenario
    (array = $arr:expr, index = $_idx:expr; ) => { { $arr } };
    // Multiple Reserved fields scenario
    (array = $arr:expr, index = $idx:expr; $desc:literal: $count:expr, $($tail:tt)*) => {
        {
            let mut arr = layout!(array = $arr, index = $idx + $count; $($tail)*);
            let mut i = $idx;
            while i < $idx + $count {
                arr[i] = FlagType::Reserved($desc);
                i += 1;
            }
            arr

        }
    };
    // Short and long description of Significant field
    (array = $arr:expr, index = $idx:expr; $short:literal $long:literal, $($tail:tt)*) => {
        {
            let mut arr = layout!(array = $arr, index = $idx + 1; $($tail)*);
            arr[$idx] = FlagType::Significant($short, $long);
            arr
        }
    };
    // Only meaningful text of Significant field
    (array = $arr:expr, index = $idx:expr; $short:literal, $($tail:tt)*) => {
        {
            let mut arr = layout!(array = $arr, index = $idx + 1; $($tail)*);
            arr[$idx] = FlagType::Significant($short, $short);
            arr
        }
    };
}


impl<'a> fmt::Display for Flag<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type_ {
            FlagType::Significant(meaning, description) => {
                if f.alternate() {
                    write!(f, "{}", description)
                } else {
                    write!(f, "{}", meaning)
                }
            },
            FlagType::Reserved(note) => {
                write!(f, "{}", note)
            },
            FlagType::Unknown => {
                write!(f, "Unknown")
            },
        }
    }
}
impl<'a> fmt::Debug for Flag<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Flag")
            .field("position", &self.position)
            .field("is_set", &self.is_set)
            .finish()
    }
}

impl<'a> Default for FlagType<'a> {
    fn default() -> Self {
        Self::Unknown
    }
}

impl<'a, T> Iter<'a, T> {
    fn new(value: T, layout: Layout<'a>) -> Self {
        Self { value, layout, index: 0 }
    }
}
impl<'a, T: Into<u128> + Copy> Iterator for Iter<'a, T> {
    type Item = Flag<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let len = core::mem::size_of::<T>() * 8;
        if self.index == len {
            None
        } else {
            let is_set = (self.value.into() & (1 << self.index)) != 0;
            let p = self.index;
            self.index += 1;
            Some(Flag { position: Position(p), is_set, type_: self.layout[p] })
        }
    }
}

impl Deref for Position {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl FromIterator<Position> for u8 {
    fn from_iter<I: IntoIterator<Item = Position>>(iter: I) -> Self {
        iter.into_iter().fold(0, |acc, p| acc | (1 << p.deref()))
    }
}
impl FromIterator<Position> for u16 {
    fn from_iter<I: IntoIterator<Item = Position>>(iter: I) -> Self {
        iter.into_iter().fold(0, |acc, p| acc | (1 << p.deref()))
    }
}
impl FromIterator<Position> for u32 {
    fn from_iter<I: IntoIterator<Item = Position>>(iter: I) -> Self {
        iter.into_iter().fold(0, |acc, p| acc | (1 << p.deref()))
    }
}
impl FromIterator<Position> for u64 {
    fn from_iter<I: IntoIterator<Item = Position>>(iter: I) -> Self {
        iter.into_iter().fold(0, |acc, p| acc | (1 << p.deref()))
    }
}
impl FromIterator<Position> for u128 {
    fn from_iter<I: IntoIterator<Item = Position>>(iter: I) -> Self {
        iter.into_iter().fold(0, |acc, p| acc | (1 << p.deref()))
    }
}
impl FromIterator<Position> for usize {
    fn from_iter<I: IntoIterator<Item = Position>>(iter: I) -> Self {
        iter.into_iter().fold(0, |acc, p| acc | (1 << p.deref()))
    }
}
impl<'a> FromIterator<&'a Position> for u8 {
    fn from_iter<I: IntoIterator<Item = &'a Position>>(iter: I) -> Self {
        iter.into_iter().fold(0, |acc, p| acc | (1 << p.deref()))
    }
}
impl<'a> FromIterator<&'a Position> for u16 {
    fn from_iter<I: IntoIterator<Item = &'a Position>>(iter: I) -> Self {
        iter.into_iter().fold(0, |acc, p| acc | (1 << p.deref()))
    }
}
impl<'a> FromIterator<&'a Position> for u32 {
    fn from_iter<I: IntoIterator<Item = &'a Position>>(iter: I) -> Self {
        iter.into_iter().fold(0, |acc, p| acc | (1 << p.deref()))
    }
}
impl<'a> FromIterator<&'a Position> for u64 {
    fn from_iter<I: IntoIterator<Item = &'a Position>>(iter: I) -> Self {
        iter.into_iter().fold(0, |acc, p| acc | (1 << p.deref()))
    }
}
impl<'a> FromIterator<&'a Position> for u128 {
    fn from_iter<I: IntoIterator<Item = &'a Position>>(iter: I) -> Self {
        iter.into_iter().fold(0, |acc, p| acc | (1 << p.deref()))
    }
}
impl<'a> FromIterator<&'a Position> for usize {
    fn from_iter<I: IntoIterator<Item = &'a Position>>(iter: I) -> Self {
        iter.into_iter().fold(0, |acc, p| acc | (1 << p.deref()))
    }
}

impl<'a, T> Significants<'a, T> {
    fn new(iter: Iter<'a, T>) -> Self {
        Self(iter)
    }
}
impl<'a, T: Into<u128> + Copy> Iterator for Significants<'a, T> {
    type Item = Flag<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(f) = self.0.next() {
            if matches!(f.type_, FlagType::Reserved(_)) || !f.is_set {
                continue;
            }
            return Some(f);
        }
        None
    }
}

impl<'a, T> Reserved<'a, T> {
    fn new(iter: Iter<'a, T>) -> Self {
        Self { iter, desc: None, start: 0 }
    }
}
impl<'a, T: Into<u128> + Copy + fmt::Debug> Iterator for Reserved<'a, T> {
    type Item = ReservedRange<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut end = 0;
        while let Some(Flag { position: Position(p), type_, .. }) = self.iter.next() {
            match (type_, self.desc) {
                (FlagType::Reserved(s), Some(desc)) => {
                    self.desc = Some(s);
                    if s != desc {
                        let start = self.start;
                        self.start = p;
                        return Some(ReservedRange { description: desc, range: start..=end });
                    }
                },
                (FlagType::Reserved(s), None) => {
                    self.desc = Some(s);
                    self.start = p;
                },
                (_, Some(desc)) => {
                    self.desc = None;
                    return Some(ReservedRange { description: desc, range: self.start..=end });
                },
                (_, None) => {
                    self.desc = None;
                },
           }
           end = p;
        }
        self.desc.take().map(|description| ReservedRange { description, range: self.start..=end })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::v1::*;
    const INDEX_SAMPLE: &[usize] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
        61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127
    ];
    layout!(
        length = 8;
        "A" "A Long",
        "B" "B Long",
        "Reserved 1": 1,
        "C" "C Long",
        "D" "D Long",
        "E" "E Long",
        "Reserved 2": 2,
    );
    #[test]
    fn layout_macro() {
        let sample = [
            FlagType::Significant("A", "A Long"),
            FlagType::Significant("B", "B Long"),
            FlagType::Reserved("Reserved 1"),
            FlagType::Significant("C", "C Long"),
            FlagType::Significant("D", "D Long"),
            FlagType::Significant("E", "E Long"),
            FlagType::Reserved("Reserved 2"),
            FlagType::Reserved("Reserved 2")
        ];
        assert_eq!(sample, LAYOUT);
    }
    #[test]
    fn iter() {
        let iter = Iter::new(0b1010_1001u8, LAYOUT);
        let sample = vec![
            (0, true,   LAYOUT[0]),
            (1, false,  LAYOUT[1]),
            (2, false,  LAYOUT[2]),
            (3, true,   LAYOUT[3]),
            (4, false,  LAYOUT[4]),
            (5, true,   LAYOUT[5]),
            (6, false,  LAYOUT[6]),
            (7, true,   LAYOUT[7]),
        ];
        for i in iter {
            println!("{:?}", i);
        }
        assert_eq!(8, iter.count(), "BYTE setted flags count");
        assert_eq!(sample, iter.map(|v| (*v.position, v.is_set, v.type_)).collect::<Vec<_>>(), "As triple vec");
    }
    
    #[test]
    fn significants() {
        let iter = Significants::new(Iter::new(0b1010_1001u8, LAYOUT));
        let meanings = vec![ "A", "C", "E" ];
        let descriptions = vec![ "A Long", "C Long", "E Long" ];
        assert_eq!(meanings, iter.map(|v| format!("{}", v)).collect::<Vec<_>>(), "Meanings");
        assert_eq!(descriptions, iter.map(|v| format!("{:#}", v)).collect::<Vec<_>>(), "Descriptions");
    }
    
    #[test]
    fn reserved() {
        let layout = &layout!(
            array = [FlagType::Unknown; 8], index = 0;
            "S A" "A Long",
            "S B" "B Long",
            "S C" "C Long",
            "S D" "D Long",
            "S E" "E Long",
            "S F" "F Long",
            "S G" "G Long",
            "S H" "H Long",
        );
        let iter = Reserved::new(Iter::new(u8::MAX, layout));
        assert_eq!(0, iter.count(), "Empty");

        let layout = &layout!(
            array = [FlagType::Unknown; 8], index = 0;
            "R 1": 8,
        );
        let sample = vec![0..=7];
        let iter = Reserved::new(Iter::new(u8::MAX, layout));
        assert_eq!(sample, iter.map(|v| v.range).collect::<Vec<_>>(), "Full");

        let layout = &layout!(
            array = [FlagType::Unknown; 8], index = 0;
            "S A" "A Long",
            "S B" "B Long",
            "S C" "C Long",
            "S D" "D Long",
            "S E" "E Long",
            "S F" "F Long",
            "R 1": 2,
        );
        let sample = vec![6..=7];
        let iter = Reserved::new(Iter::new(u8::MAX, layout));
        assert_eq!(sample, iter.map(|v| v.range).collect::<Vec<_>>(), "Simple");

        let layout = &layout!(
            array = [FlagType::Unknown; 16], index = 0;
            "S A" "A Long",
            "S B" "B Long",
            "R 1": 1,
            "S C" "C Long",
            "S C" "C Long",
            "S D" "D Long",
            "S E" "E Long",
            "R 2": 2,
            "S C" "C Long",
            "R 2": 2,
            "R 3": 4,
        );
        let sample = vec![
            2..=2,
            7..=8,
            10..=11,
            12..=15,
        ];
        let iter = Reserved::new(Iter::new(u16::MAX, layout));
        assert_eq!(sample, iter.map(|v| v.range).collect::<Vec<_>>(), "Complex");
    }
    
    #[test]
    #[should_panic(expected = "attempt to shift left with overflow")]
    fn from_iterator_shift_overflow() {
        let _ = INDEX_SAMPLE.iter().map(|&p| Position(p)).collect::<u8>();
    }
    #[test]
    fn from_iterator_values() {
        let a = 0b1010_1100u8;
        let b = INDEX_SAMPLE.iter().take_while(|&&p| p < 8).map(|&p| Position(p)).collect();
        assert_eq!(a, b, "u8:\n{:08b}\n{:08b}", a, b);
        let a = 0b0010_1000_1010_1100u16;
        let b = INDEX_SAMPLE.iter().take_while(|&&p| p < 16).map(|&p| Position(p)).collect();
        assert_eq!(a, b, "u16:\n{:016b}\n{:016b}", a, b);
        let a = 2693408940u32;
        let b = INDEX_SAMPLE.iter().take_while(|&&p| p < 32).map(|&p| Position(p)).collect();
        assert_eq!(a, b, "u32:\n{:032b}\n{:032b}", a, b);
        let a = 2891462833508853932u64;
        let b = INDEX_SAMPLE.iter().take_while(|&&p| p < 64).map(|&p| Position(p)).collect();
        assert_eq!(a, b, "u64:\n{:064b}\n{:064b}", a, b);
        let a = 170152392186162032610075049835446806700u128;
        let b = INDEX_SAMPLE.iter().take_while(|&&p| p < 128).map(|&p| Position(p)).collect();
        assert_eq!(a, b, "u128:\n{:0128b}\n{:0128b}", a, b);
    }
}
