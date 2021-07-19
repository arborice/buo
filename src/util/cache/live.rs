use crate::prelude::*;
// use std::{collections::HashMap, path::Path};
use tinyvec::ArrayVec;

pub struct LiveCache {
    cache: ArrayVec<[MediaMeta; 30]>,
}
