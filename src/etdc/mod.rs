use itertools::Itertools;

pub mod characters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Attribute {
    Cunning,
    Might,
    Wisdom,
}

#[derive(Copy, Clone, Debug)]
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

    pub fn is_double(&self) -> bool {
        match self {
            Face::Double(_) => true,
            _ => false,
        }
    }
}

pub type Die = [Face; 6];

pub struct Rolls<'a>(Vec<Roll<'a>>);
pub struct Roll<'a>(Vec<&'a Face>);

impl<'a> Rolls<'a> {
    pub fn times<'b>(die: &'b Die, times: usize) -> Rolls<'b> {
        let mut dice = Vec::new();
        dice.resize(times, die);

        Rolls::dice(&dice)
    }

    pub fn dice<'b>(dice: &[&'b Die]) -> Rolls<'b> {
        Rolls(
            dice.iter()
                .map(|die| die.iter())
                .multi_cartesian_product()
                .map(|v| Roll(v))
                .collect_vec(),
        )
    }

    pub fn probability_of<F>(&self, f: F) -> (usize, usize, f64)
    where
        F: FnMut(&&Roll) -> bool,
    {
        let num_outcomes = self.len();
        let num_matches = self.iter().filter(f).count();

        (
            num_outcomes,
            num_matches,
            num_matches as f64 / num_outcomes as f64,
        )
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> impl Iterator<Item = &Roll> {
        self.0.iter()
    }
}

impl<'a> Roll<'a> {
    pub fn count_attr(&self, attr: Attribute) -> usize {
        self.0.iter().map(|r| r.num_matches(attr)).sum()
    }

    pub fn has_double(&self) -> bool {
        self.0.iter().any(|f| f.is_double())
    }
}
