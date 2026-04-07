use crate::atom::Atom;
use crate::element::Element;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Molecule<'a> {
    // Stores elements with their natural isotpoic abudnances
    pub elements: &'a [(&'static Element, usize)],
    // Stores atoms with certain isotpoic abudnances
    pub atoms:    &'a [(&'static Atom,    usize)],
}

impl<'a> Molecule<'a> {
    pub fn new(
        elements: &'a [(&'static Element, usize)], 
        atoms: &'a [(&'static Atom, usize)],
    ) -> Self {
        Self { elements, atoms }
    }

    pub fn avg_amu(&self) -> f64 {
        let e_avg_amu: f64 = self.elements.iter().map(|&(element, count)| element.avg_amu() * count as f64).sum();
        let a_amu: f64 = self.atoms.iter().map(|&(atom, count)| atom.amu * count as f64).sum();

        e_avg_amu + a_amu
    }

    pub fn amu(&self) -> f64 {
        let e_amu: f64 = self.elements.iter().map(|&(element, count)| element.amu() * count as f64).sum();
        let a_amu: f64 = self.atoms.iter().map(|&(atom, count)| atom.amu * count as f64).sum();

        e_amu + a_amu
    }
}