use super::super::Style;
use rand::{distributions::Alphanumeric, rngs::SmallRng, Rng, SeedableRng};

pub type DomNode = ();

pub fn classname_entropy() -> impl std::fmt::Display {
    SmallRng::from_entropy()
        .sample_iter(Alphanumeric)
        .take(30)
        .collect::<String>()
}

impl Style {
    pub fn mount(&mut self) {}
    pub fn unmount(&mut self) {}
}
