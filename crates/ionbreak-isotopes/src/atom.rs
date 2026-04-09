pub struct Atom {
    // Atomic number
    pub num: usize,
    // Atomic mass
    pub mass: usize,
    // Monoisotopic mass (amu)
    pub amu: f64,
    // Isotopic probability (0.0 - 1.0)
    pub prob: f64,
}

impl Atom {
    pub const fn new(num: usize, mass: usize, amu: f64, prob: f64) -> Self {
        Self { num, mass, amu, prob }
    }
}
