use std::collections::HashMap;

use miniquad::Bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetType {
    Food,
    Snake,
    Tail,
}

pub type BindingAssets = HashMap<AssetType, Bindings>;
