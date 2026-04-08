use std::env;
use ionbreak_isotopes::isotope::*;
use ionbreak_isotopes::isotope::natural::*;
use ionbreak_isotopes::molecule::Molecule;
use ionbreak_spectrum::isodalton::isotope_distribution;

fn main() {
    let args: Vec<String> = env::args().collect();
    let max_peaks: usize = args[1].parse::<usize>().unwrap();

    let elements = [(&C,254), (&H,189), (&N,65), (&O,75), (&S,6)];
    let atoms = [(&H2,189)];
    let mol: Molecule = Molecule::new(
        &elements,
        &atoms
    );

    let peaks = isotope_distribution(&mol, max_peaks);

    println!("mass,prob");
    for peak in peaks {
        println!("{:},{:}", peak.amu, peak.prob);
    }
}