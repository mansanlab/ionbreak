use crate::atom::Atom;
use crate::element::Element;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Molecule {
    pub atoms: Vec<Atom>
}

impl Molecule {

    pub fn new_from_formula(
        element_formula: &[(Element, usize)],
        isotope_formula: &[(Atom, usize)]
    ) -> Self {
        let mut atoms: Vec<Atom> = Vec::new();

        for (element, count) in element_formula {
            for &atom in element.atoms {
                for _ in 0..*count {
                    atoms.push(atom);
                }
            }
        }

        for (atom, count) in isotope_formula {
            let mut atom= atom.clone();
            atom.prob = 1.0 as f64;
            for _ in 0..*count {
                atoms.push(atom);
            }
        }

        Self { atoms }
    }

    pub fn new_from_element_formula(formula: &[(Element, usize)]) -> Self {
        Self::new_from_formula(formula, &[])
    }

    pub fn new_from_isotope_formula(formula: &[(Atom, usize)]) -> Self {
        Self::new_from_formula(&[], formula)
    }

    pub fn new_from_elements(elements: &[Element]) -> Self {
        let mut atoms: Vec<Atom> = Vec::new();

        for element in elements {
            for atom in element.atoms {
                atoms.push(atom.clone());
            }
        }

        Self { atoms }
    }

    pub fn add_formula(
        &mut self,
        element_formula: &[(Element, usize)],
        isotope_formula: &[(Atom, usize)]
    ) {
        for (element, count) in element_formula {
            for &atom in element.atoms {
                for _ in 0..*count {
                    self.atoms.push(atom);
                }
            }
        }

        for (atom, count) in isotope_formula {
            let mut atom= atom.clone();
            atom.prob = 1.0 as f64;
            for _ in 0..*count {
                self.atoms.push(atom);
            }
        }

    }

    pub fn add_element_formula(&mut self, formula: &[(Element, usize)]) {
        self.add_formula(formula, &[]);
    }

    pub fn add_isotope_formula(&mut self, formula: &[(Atom, usize)]) {
        self.add_formula(&[], formula);
    }

    pub fn add_elements(&mut self, elements: &[Element]) {
        for element in elements {
            for &atom in element.atoms {
                self.atoms.push(atom);
            }
        }
    }
    
    pub fn amu(&self) -> f64 {
        self.atoms.iter().map(|&a| a.amu * a.prob).sum()
    }
}