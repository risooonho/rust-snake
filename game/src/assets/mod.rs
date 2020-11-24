use std::collections::HashMap;

use miniquad::Bindings;

#[derive(PartialEq, Eq, Hash)]
pub enum AssetType {
    Food,
    Snake,
    Tail,
}

pub type BindingAssets = HashMap<AssetType, Bindings>;
