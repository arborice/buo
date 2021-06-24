use crate::prelude::*;

pub trait ExtCallback: Send + Sync {
    fn try_get_meta(self, path: &std::path::Path) -> Result<MediaMeta>;
}

// pub trait Bookmark {
// type Meta;
// fn create_bookmark(&self, meta: Self::Meta) -> Result<()>;

// type Ident;
// fn delete_bookmark(&self, id: Self::Ident) -> Result<()>;
// }
