use rand::Rng;
use rand::rngs::ThreadRng;

pub fn rand_range(rng: &mut ThreadRng, a: i16, b: i16) -> i16 {
    rng.gen_range(a..b)
}

pub fn bernoulli(rng: &mut ThreadRng, p: f32) -> bool {
    rand_range(rng, 0, 1000) < ((1000. * p) as i16)
}

