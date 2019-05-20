use crate::etdc::characters::MILLER;
use crate::etdc::Attribute::Cunning;
use crate::etdc::Rolls;

mod etdc;

fn main() {
    let t = Rolls::times(MILLER, 3).probability_of(|r| r.count_attr(Cunning) >= 2);
    println!("{:?}", t);
}
