use terminal_size::{Width, terminal_size};

pub fn get_term_width() -> u16 {
    let (Width(w), _) = terminal_size().unwrap();
    return w
}