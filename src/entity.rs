#[derive(PartialEq, Eq, Clone, Debug)]

pub enum Entity {
    Player,
    Enemy(usize),
    Projectile(usize),
}
