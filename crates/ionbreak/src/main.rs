use ionbreak::isotope::*; // Loads all Atoms
use ionbreak::isotope::natural::*; // Loads all Elements
use ionbreak::molecule::Molecule;

fn main() {
    let samarium_153_sm_lexidronam: Molecule = Molecule::new_from_formula(
        &[(H, 12), (C, 6), (N, 2), (O, 12), (P, 4), (NA, 5)],
        &[(SM153, 1)]
    );

    println!("{}", samarium_153_sm_lexidronam.amu());
}