use crate::trump::{Suit, Trump};

enum Role {
    Napoleon,
    Aide,
    Union,
}

struct Declaration {
    suit: Option<Suit>,
    number: usize,
    aide: Trump,
}


