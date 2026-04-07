#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Atom {
    // Atomic number
    pub num: u16,
    // Atomic mass
    pub mass: u16,
    // Monoisotopic mass (amu)
    pub amu: f64,
    // Isotopic probability (0.0 - 1.0)
    pub prob: f64,
}

impl Atom {
    pub const fn new(num: u16, mass: u16, amu: f64, prob: f64) -> Self {
        Self { num, mass, amu, prob }
    }
}
