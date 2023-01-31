use std::rc::Rc;

use yew::prelude::*;

use crate::sudoku::digit::Digit;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct View {
    pub focus_digit: Option<Digit>,
}

pub enum ViewAction {
    Reset,
    SetFocus(Option<Digit>),
    ShowStrategy,
}

impl Reducible for View {
    type Action = ViewAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ViewAction::Reset => Self { focus_digit: None }.into(),
            ViewAction::SetFocus(digit) => {
                let new_focus = if digit != self.focus_digit {
                    digit
                } else {
                    None
                };

                Self {
                    focus_digit: new_focus,
                }
                .into()
            }
            ViewAction::ShowStrategy => {
                unimplemented!()
            }
        }
    }
}
