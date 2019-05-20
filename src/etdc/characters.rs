use super::Attribute::*;
use super::Die;
use super::Face::*;

pub const ABBOT: &Die = &[
    Single(Wisdom),
    Double(Wisdom),
    Single(Wisdom),
    Double(Might),
    Single(Might),
    Single(Cunning),
];

pub const COOK: &Die = &[
    Single(Might),
    Double(Might),
    Single(Might),
    Double(Cunning),
    Single(Cunning),
    Single(Wisdom),
];

pub const MILLER: &Die = &[
    Single(Cunning),
    Double(Cunning),
    Single(Cunning),
    Double(Might),
    Single(Might),
    Single(Wisdom),
];

pub const SMITH: &Die = &[
    Single(Might),
    Double(Might),
    Single(Might),
    Double(Wisdom),
    Single(Wisdom),
    Single(Cunning),
];

pub const TANNER: &Die = &[
    Single(Wisdom),
    Double(Wisdom),
    Single(Wisdom),
    Double(Cunning),
    Single(Cunning),
    Single(Might),
];

pub const TAILOR: &Die = &[
    Single(Cunning),
    Double(Cunning),
    Single(Cunning),
    Double(Wisdom),
    Single(Wisdom),
    Single(Might),
];
