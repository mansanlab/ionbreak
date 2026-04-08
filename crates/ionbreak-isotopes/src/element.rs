use crate::atom::Atom;

pub struct Element {
    // Isotopes of Element
    pub atoms: &'static [&'static Atom],
}

impl Element {
    pub const fn new(atoms: &'static [&'static Atom]) -> Self {
        Self { atoms }
    }

    pub fn avg_amu(&self) -> f64 {
        let w_amu: f64 = self.atoms.iter().map(|a| a.amu * a.prob).sum();
        let w: f64 = self.atoms.iter().map(|a| a.prob).sum();

        w_amu / w
    }

    pub fn amu(&self) -> f64 {
        // relies on `isotope::natural` to be sorted by most abundant isotope first
        self.atoms.first().unwrap().amu
    }

    pub fn num(&self) -> u16 {
        self.atoms.first().unwrap().num
    }

    pub fn mass(&self) -> u16 {
        // relies on `isotope::natural` to be sorted by most abundant isotope first
        self.atoms.first().unwrap().mass
    }
}