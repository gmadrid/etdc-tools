use crate::etdc::characters::MILLER;
use crate::etdc::Attribute::Cunning;
use crate::etdc::{Attribute, Die, Face};
use itertools::Itertools;

mod etdc;

fn roll_dice(dice: &[Die]) -> Vec<Vec<&Face>> {
    let foo = dice.iter().map(|d| d.iter()).multi_cartesian_product();
    let q = foo.collect_vec();
    println!("ROLL LEN: {}", q.len());
    println!("ROLL_DICE: {:?}", q);
    q
}

fn count_attrs(roll: &[&Face; 3], attr: Attribute) -> usize {
    roll.iter().map(|r| r.num_matches(attr)).sum()
}

fn foo_slice<T>(slice: &[T]) {
    for (i, _) in slice.iter().enumerate() {
        println!("{}", i)
    }
}

trait CrossProduct {
    type ArrayType;

    fn cross_product(self) -> Self::ArrayType;
}

impl<T> CrossProduct for (T, T, T) {
    type ArrayType = [T; 3];

    fn cross_product(self) -> Self::ArrayType {
        [self.0, self.1, self.2]
    }
}

fn main() {
    let rolls = roll_dice(&[MILLER, MILLER, MILLER]);

    let who = MILLER;

    //let mut rolls = Vec::default();

    let count = rolls.iter().filter(|r| r.count_attrs(Cunning) >= 2).count();

    println!(
        "{} out of {} ({}%)",
        count,
        rolls.len(),
        100f64 * count as f64 / rolls.len() as f64
    );
}
