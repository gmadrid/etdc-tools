pub mod characters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Attribute {
    Cunning,
    Might,
    Wisdom,
}

#[derive(Debug)]
pub enum Face {
    Single(Attribute),
    Double(Attribute),
}

impl Face {
    pub fn num_matches(&self, attr: Attribute) -> usize {
        match self {
            Face::Single(rattr) if attr == *rattr => 1,
            Face::Double(rattr) if attr == *rattr => 2,
            _ => 0,
        }
    }
}

pub type Die = [Face; 6];

struct Rolls<'a>([&'a Face]);

impl<'a> Rolls<'a> {
    fn count_attrs(&self, attr: Attribute) -> usize {
        self.0.iter().map(|r| r.num_matches(attr)).sum()
    }
}
