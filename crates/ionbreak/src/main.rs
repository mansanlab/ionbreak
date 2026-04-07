use ionbreak_isotopes::isotope::*;
use ionbreak_isotopes::isotope::natural::*; // Loads all Elements
use ionbreak_isotopes::molecule::Molecule;

fn main() {
    
    let element_formula = [(&C,41), (&H,73), (&N,1), (&O,8), (&P,1)];
    let atom_formula = [(&H2, 7)];

    let lipid_15_0_18_1_d7_pc: Molecule = Molecule::new(
    &element_formula,
    &atom_formula
    );

    println!("{}", lipid_15_0_18_1_d7_pc.avg_amu());
    println!("{}", lipid_15_0_18_1_d7_pc.amu());

    println!("{:p}", &C);
    println!("{:p}", element_formula[0].0);
}