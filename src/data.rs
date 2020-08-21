#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Angle(pub i16, pub u8, pub u8);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up, Down, Left, Right
}
