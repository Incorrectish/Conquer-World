#[derive(PartialEq, Eq, Clone, Debug, serde::Serialize, serde::Deserialize)]

pub enum Entity {
    Player,
    Enemy,
    Projectile,
}
