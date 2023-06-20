pub(crate) use std::sync::Arc;
pub(crate) use model::Recipe;
pub(crate) use crate::model;
pub(crate) use futures::StreamExt;
pub(crate) use crate::config::PARALLELISM_FACTOR;

pub type AStr = Arc<str>;