use crate::graphics::renderer::AssetIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetType {
    Food,
    Snake,
    Tail,
}

impl Into<AssetIdentity> for AssetType {
    fn into(self) -> AssetIdentity {
        match self {
            AssetType::Food => "Food".into(),
            AssetType::Snake => "Snake".into(),
            AssetType::Tail => "Tail".into(),
        }
    }
}
