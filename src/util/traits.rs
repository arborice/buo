use crate::prelude::*;

pub trait Bookmark {
    type Meta;
    fn create_bookmark(&self, meta: Self::Meta) -> Result<()>;

    type Ident;
    fn delete_bookmark(&self, id: Self::Ident) -> Result<()>;
}
