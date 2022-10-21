use crate::contracts::TuiPwSafeErrors::{StoreFileNotFound, StoreFileNotRead, UnknownError};
use crate::SafeModel;
use crossterm::event::KeyEvent;
use rs_pwsafe::pwserrors::PwSafeError;
use std::fmt::{Display, Formatter};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

// TuiPwSafeError
pub enum TuiPwSafeErrors {
    StoreFileNotFound, //:&str = "PwSafe Store file not specified",
    StoreFileNotRead,
    UnknownError(PwSafeError),
}

impl Display for TuiPwSafeErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreFileNotFound => write!(f, "PwSafe Store file not specified"),
            StoreFileNotRead => write!(f, "Store file is not read"),
            UnknownError(e) => write!(f, "Unexpected Error {:?}", e),
        }
    }
}

impl From<PwSafeError> for TuiPwSafeErrors {
    fn from(error: PwSafeError) -> Self {
        match error {
            PwSafeError::FileNotFound => StoreFileNotFound,
            PwSafeError::InvalidKey => StoreFileNotRead,
            other => UnknownError(other),
        }
    }
}

// Traits
pub trait UiWidgetVm<B: Backend> {
    fn capture_key(&mut self, key: KeyEvent);
    fn draw(&mut self, f: &mut Frame<B>, rec: Rect);
    fn update_model(&mut self, model: &mut SafeModel);
    fn is_done(&self) -> bool;
}

// Const functions
#[allow(dead_code)]
pub const fn type_eq<T: ?Sized, U: ?Sized>() -> bool {
    trait TypeEq<U: ?Sized> {
        const VALUE: bool;
    }
    impl<T: ?Sized, U: ?Sized> TypeEq<U> for T {
        default const VALUE: bool = false;
    }

    impl<T: ?Sized> TypeEq<T> for T {
        const VALUE: bool = true;
    }
    <T as TypeEq<U>>::VALUE
}
