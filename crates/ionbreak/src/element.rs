use crate::atom::Atom;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Element {
    // Isotopes of Element
    pub isotopes: &'static [Atom],
}

impl Element {
    pub const fn new(isotopes: &'static [Atom]) -> Self {
        Self { isotopes }
    }

    pub fn amu(&self) -> f64 {
        self.isotopes.iter().map(|a| a.amu * a.prob).sum()
    }

    pub fn num(&self) -> u16 {
        self.isotopes.first().unwrap().num
    }

    pub fn mass(&self) -> u16 {
        // relies on `isotope::natural` and `isotope::all` to be sorted by most abundant isotope first
        self.isotopes.first().unwrap().mass
    }
}
