use self::Direction::*;
use std::slice::Iter;

#[derive(Debug)]
pub enum Direction {
    forward,
    left,
    backward,
    right,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [forward, left, backward, right];
        DIRECTIONS.iter()
    }
}

fn main() {
    for dir in Direction::iterator() {
        println!("{:?}", dir);
    }
}