use leptos::prelude::*;
use std::sync::Arc;

use crate::framework::Framework;

pub struct Leptos;

impl Framework for Leptos {
    type Context = Arc<dyn ErasedDestructor>;
}

/// A dumy trait just to get some sort of `dyn Drop`
pub trait ErasedDestructor: 'static {}
/// The only think we're interested about the handle is its [`Drop`] impl, so we erase it
/// to avoid generics in the [`Framework`] struct
impl<T: Mountable + 'static> ErasedDestructor for UnmountHandle<T> {}
