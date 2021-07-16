use crate::prelude::*;

pub trait ExtCallback: Send + Sync {
    fn try_get_meta(&self, path: &std::path::Path) -> Result<Option<MediaMeta>>;
}

pub trait IntoMeta {
    fn into_meta(self, file_name: String) -> Option<MediaMeta>;
}

/// Trait to convert a bool type to an Option<T> by providing the T in the `opt` method
pub trait OptionFromBool<T> {
    /// Returns a Some(T) if self is true, or else returns None
    fn opt(self, some_inner: T) -> Option<T>;
    /// Similar to `Option::and`, this will return optb if self is true
    fn opt_and(self, optb: Option<T>) -> Option<T>;
    /// Return `inner` if self is false
    fn opt_not(self, inner: T) -> Option<T>;
}

impl<T> OptionFromBool<T> for bool {
    fn opt(self, some_inner: T) -> Option<T> {
        if self {
            Some(some_inner)
        } else {
            None
        }
    }

    fn opt_and(self, optb: Option<T>) -> Option<T> {
        if self {
            optb
        } else {
            None
        }
    }

    fn opt_not(self, inner: T) -> Option<T> {
        if !self {
            Some(inner)
        } else {
            None
        }
    }
}
