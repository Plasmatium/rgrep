use std::borrow::Cow::{self, Borrowed};

use terminal_size::{terminal_size, Width};

pub fn get_term_width() -> u16 {
    let (Width(w), _) = terminal_size().unwrap();
    return w;
}

pub fn is_borrowed<B: ?Sized + ToOwned>(c: &Cow<B>) -> bool {
    match c {
        Borrowed(_) => true,
        _ => false,
    }
}
