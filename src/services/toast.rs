use std::rc::Rc;

use js_sys::Date;
use web_sys::{window, Storage, Window};
use yew::{Reducible, UseReducerHandle};

use crate::models::{AppError, Settings};

#[derive(Clone, PartialEq, Debug)]
pub struct Toast {
    pub id: u64,
    pub message: Box<str>,
    pub kind: ToastKind,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ToastKind {
    Success,
    Error,
    Info,
}

pub enum ToastAction {
    Add(Toast),
    Remove(u64),
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct ToastState {
    pub toasts: Vec<Toast>,
}

impl Reducible for ToastState {
    type Action = ToastAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ToastAction::Add(toast) => {
                let mut toasts = self.toasts.clone();
                toasts.push(toast);
                Self { toasts }.into()
            }
            ToastAction::Remove(id) => {
                let toasts = self
                    .toasts
                    .iter()
                    .cloned()
                    .filter(|t| t.id != id)
                    .collect();

                Self { toasts }.into()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToastManager {
    dispatch: UseReducerHandle<ToastState>,
}

impl ToastManager {
    pub fn new(dispatch: UseReducerHandle<ToastState>) -> Self {
        Self { dispatch }
    }

    pub fn get_toasts(&self) -> &[Toast] {
        &self.dispatch.toasts
    }

    pub fn send(&self, error: AppError) {
        let toast = Toast {
            id: Date::now() as u64,
            message: error.message.into(),
            kind: ToastKind::Error,
        };

        self.dispatch.dispatch(ToastAction::Add(toast));
    }

    pub fn success(&self, msg: impl Into<Box<str>>) {
        let toast = Toast {
            id: Date::now() as u64,
            message: msg.into(),
            kind: ToastKind::Success,
        };

        self.dispatch.dispatch(ToastAction::Add(toast));
    }

    pub fn remove(&self, id: u64) {
        self.dispatch.dispatch(ToastAction::Remove(id));
    }
}