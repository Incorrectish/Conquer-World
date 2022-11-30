#[derive(PartialEq, Eq, Clone)]

pub enum Entity {
    Player,
    Enemy(usize),
    Projectile(usize),
}
