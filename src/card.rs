#[derive(Debug)]
pub enum Shape {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug)]
pub struct Card {
    pub shape: Shape,
    pub number: u8,
    pub face_up: bool,
}

impl Card {
    pub fn new(shape: Shape, number: u8, face_up: bool) -> Card {
        Card {
            shape,
            number,
            face_up,
        }
    }

    pub fn flip(&mut self) {
        self.face_up = !self.face_up;
    }
}
