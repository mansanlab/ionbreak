use std::fmt;
use std::collections::HashSet;
use crate::atom::Atom;
use crate::element::Element;
use crate::isotope::name::{ATOM_SYMBOL_BY_NUM, ATOM_SYMBOL_BY_NUM_MASS};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Molecule {
    pub atoms: Vec<(&'static Atom, usize, Option<f64>)>,
}

impl Molecule {
    pub fn new() -> Self {
        Self { atoms: Vec::new() }
    }

    pub fn new_from_formula(
        element_formula: &[(Element, usize)],
        isotope_formula: &[(&'static Atom, usize)],
    ) -> Self {
        let mut mol = Self::new();
        mol.add_formula(element_formula, isotope_formula);
        mol
    }

    pub fn new_from_element_formula(formula: &[(Element, usize)]) -> Self {
        Self::new_from_formula(formula, &[])
    }

    pub fn new_from_isotope_formula(formula: &[(&'static Atom, usize)]) -> Self {
        Self::new_from_formula(&[], formula)
    }

    pub fn new_from_elements(elements: &[Element]) -> Self {
        let mut mol = Self::new();
        mol.add_elements(elements);
        mol
    }

    pub fn add_formula(
        &mut self,
        element_formula: &[(Element, usize)],
        isotope_formula: &[(&'static Atom, usize)],
    ) {
        for (element, count) in element_formula {
            for atom in element.atoms {
                self.atoms.push((atom, *count, None));
            }
        }

        for (atom, count) in isotope_formula {
            self.atoms.push((atom, *count, Some(1.0)));
        }
    }

    pub fn add_element_formula(&mut self, formula: &[(Element, usize)]) {
        self.add_formula(formula, &[]);
    }

    pub fn add_isotope_formula(&mut self, formula: &[(&'static Atom, usize)]) {
        self.add_formula(&[], formula);
    }

    pub fn add_elements(&mut self, elements: &[Element]) {
        for element in elements {
            for atom in element.atoms {
                self.atoms.push((atom, 1, None));
            }
        }
    }

    pub fn amu(&self) -> f64 {
        self.atoms
            .iter()
            .map(|(a, count, prob_override)| {
                let prob = prob_override.unwrap_or(a.prob);
                a.amu * prob * *count as f64
            })
            .sum()
    }
}

impl fmt::Display for Molecule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let tot_num_atoms: usize = self.atoms
                                        .iter().map(|(_, count, _)| count).sum();

        let unique_num_atoms: usize = self.atoms
                                        .iter()
                                        .map(|(a, _, _)| a.mass)
                                        .collect::<HashSet<_>>()
                                        .len();

        let tot_num_elements: u64 = self.atoms
                                        .iter()
                                        .map(|(a, count, _)| (a.num as u64, *count as u64))
                                        .collect::<HashSet<_>>()
                                        .iter()
                                        .map(|(_, count)| *count)
                                        .sum();

        let unique_num_elements: usize = self.atoms
                                        .iter()
                                        .map(|(a, _, _)| a.num)
                                        .collect::<HashSet<_>>()
                                        .len();

        let mut chemical_formula: Vec<(u16, usize)> = self.atoms
                                        .iter()
                                        .filter(|(_, _, prob_override)| prob_override.is_none())
                                        .map(|(a, count, _)| (a.num, *count))
                                        .collect::<HashSet<_>>()
                                        .into_iter()
                                        .collect();

        chemical_formula.sort_unstable_by_key(|&(key, _)| key);
        let formula_string: String = chemical_formula
                                        .iter()
                                        .map(|(key, count)| 
                                            format!("{}{} ", ATOM_SYMBOL_BY_NUM.get(key).unwrap(), count)
                                        )
                                        .collect();

        let mut chemical_formula_isotopes: Vec<((u16, u16), usize)> = self.atoms
                                        .iter()
                                        .filter(|(_, _, prob_override)| prob_override.is_some())
                                        .map(|(a, count, _)| ((a.num, a.mass), *count))
                                        .collect::<HashSet<_>>()
                                        .into_iter()
                                        .collect();
                                              
        chemical_formula_isotopes.sort_unstable_by_key(|&(key, _)| key);

        let formula_string_isotopes: String = chemical_formula_isotopes
                                        .iter()
                                        .map(|(key, count)| 
                                            format!("{}{} ", ATOM_SYMBOL_BY_NUM_MASS.get(key).unwrap(), count)
                                        )
                                        .collect();

        write!(f, "{formula_string_isotopes}{formula_string}\n")?;
        write!(f, "Avg. mass: {:?} amu\n", self.amu())?;
        write!(f, "No. of Atoms: {tot_num_atoms}\n")?;
        write!(f, "No. of Unique Atoms: {unique_num_atoms}\n")?;
        write!(f, "No. of Elements: {tot_num_elements}\n")?;
        write!(f, "No. of Unique Elements: {unique_num_elements}\n")
    }
}