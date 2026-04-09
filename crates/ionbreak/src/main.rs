// use std::env;
// use ionbreak_isotopes::isotope::*;
use ionbreak_isotopes::isotope::natural::*;
use ionbreak_isotopes::molecule::Molecule;
use ionbreak_spectrum::spectrum::Spectrum;
use ionbreak_spectrum::isodalton::isotope_distribution;

fn main() {
    let max_peaks: usize = 15;

    let elements = [(&C, 41), (&H, 63), (&N, 11), (&O, 11)];
    let atoms = [];

    let mol: Molecule = Molecule::new(
        &elements,
        &atoms
    );

    let mut spec = Spectrum {
        peaks: isotope_distribution(&mol, max_peaks),
        z: 0
    };

    println!("mass,prob");
    for peak in &spec.peaks {
        println!("{:},{:}", peak.amu, peak.prob);
    }

    for z in -4..5 {
        if z != 0 {
            spec.z = z;
            println!("z={:}", z);
            println!("mass,prob");
            for peak in &spec.mz() {
                println!("{:},{:}", peak.amu, peak.prob);
            }
        }
    }
}