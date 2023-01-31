use super::{pos::Cell, digit::Digit};


#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct View {
    pub focus_digit: Option<Digit>
}
