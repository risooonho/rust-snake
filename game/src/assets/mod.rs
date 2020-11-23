use std::collections::HashMap;

use miniquad::Bindings;

#[derive(PartialEq, Eq, Hash)]
pub enum AssetType {
    Food,
    Snake,
}

pub type BindingAssets = HashMap<AssetType, Bindings>;
