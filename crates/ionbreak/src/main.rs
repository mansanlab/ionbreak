use ionbreak_isotopes::isotope::*;
use ionbreak_isotopes::isotope::natural::*; // Loads all Elements
use ionbreak_isotopes::molecule::Molecule;

fn main() {
    // Average mass
    let angiotensin_ii: Molecule = Molecule::new_from_element_formula(&[
        (C, 50), (H, 71), (N, 13), (O, 12), (S, 0)
    ]);
    let bovine_insulin: Molecule = Molecule::new_from_element_formula(&[
        (C, 254), (H, 377), (N, 65), (O, 75), (S, 6)
    ]);
    let human_insulin: Molecule = Molecule::new_from_element_formula(&[
        (C, 520), (H, 817), (N, 139), (O, 147), (S, 8)
    ]);
    let human_myoglobin: Molecule = Molecule::new_from_element_formula(&[
        (C, 744), (H, 1224), (N, 210), (O, 222), (S, 5)
    ]);
    let human_intrinsic_factor: Molecule = Molecule::new_from_element_formula(&[
        (C, 2023), (H, 3208), (N, 524), (O, 619), (S, 20)
    ]);
    let bovine_serum_albumin: Molecule = Molecule::new_from_element_formula(&[
        (C, 2934), (H, 4615), (N, 781), (O, 897), (S, 39)
    ]);
    let human_na_k_atpase: Molecule = Molecule::new_from_element_formula(&[
        (C, 5047), (H, 8014), (N, 1338), (O, 1495), (S, 8)
    ]);
    let human_atp: Molecule = Molecule::new_from_element_formula(&[
        (C, 8574), (H, 13378), (N, 2092), (O, 2392), (S, 77)
    ]);
    let human_intrinsic_factor_receptor: Molecule = Molecule::new_from_element_formula(&[
        (C, 17600), (H, 26474), (N, 4752), (O, 5486), (S, 197)
    ]);
    let human_dynein: Molecule = Molecule::new_from_element_formula(&[
        (C, 23832), (H, 37816), (N, 6528), (O, 7031), (S, 170)
    ]);

    for mol in &[
        angiotensin_ii,
        bovine_insulin,
        human_insulin,
        human_myoglobin,
        human_intrinsic_factor,
        bovine_serum_albumin,
        human_na_k_atpase,
        human_atp,
        human_intrinsic_factor_receptor,
        human_dynein
    ] {
        println!("{}\n\n", mol);
    }


    // Monoisotopic mass
    let angiotensin_ii: Molecule = Molecule::new_from_isotope_formula(&[
        (&C12, 50), (&H1, 71), (&N14, 13), (&O16, 12), (&S32, 0)
    ]);
    let bovine_insulin: Molecule = Molecule::new_from_isotope_formula(&[
        (&C12, 254), (&H1, 377), (&N14, 65), (&O16, 75), (&S32, 6)
    ]);
    let human_insulin: Molecule = Molecule::new_from_isotope_formula(&[
        (&C12, 520), (&H1, 817), (&N14, 139), (&O16, 147), (&S32, 8)
    ]);
    let human_myoglobin: Molecule = Molecule::new_from_isotope_formula(&[
        (&C12, 744), (&H1, 1224), (&N14, 210), (&O16, 222), (&S32, 5)
    ]);
    let human_intrinsic_factor: Molecule = Molecule::new_from_isotope_formula(&[
        (&C12, 2023), (&H1, 3208), (&N14, 524), (&O16, 619), (&S32, 20)
    ]);
    let bovine_serum_albumin: Molecule = Molecule::new_from_isotope_formula(&[
        (&C12, 2934), (&H1, 4615), (&N14, 781), (&O16, 897), (&S32, 39)
    ]);
    let human_na_k_atpase: Molecule = Molecule::new_from_isotope_formula(&[
        (&C12, 5047), (&H1, 8014), (&N14, 1338), (&O16, 1495), (&S32, 8)
    ]);
    let human_atp: Molecule = Molecule::new_from_isotope_formula(&[
        (&C12, 8574), (&H1, 13378), (&N14, 2092), (&O16, 2392), (&S32, 77)
    ]);
    let human_intrinsic_factor_receptor: Molecule = Molecule::new_from_isotope_formula(&[
        (&C12, 17600), (&H1, 26474), (&N14, 4752), (&O16, 5486), (&S32, 197)
    ]);
    let human_dynein: Molecule = Molecule::new_from_isotope_formula(&[
        (&C12, 23832), (&H1, 37816), (&N14, 6528), (&O16, 7031), (&S32, 170)
    ]);

    for mol in &[
        angiotensin_ii,
        bovine_insulin,
        human_insulin,
        human_myoglobin,
        human_intrinsic_factor,
        bovine_serum_albumin,
        human_na_k_atpase,
        human_atp,
        human_intrinsic_factor_receptor,
        human_dynein
    ] {
        println!("{}\n\n", mol);
    }



    let lipid_15_0_18_1_d7_pc: Molecule = Molecule::new_from_formula(
    &[(C,41), (H,73), (N,1), (O,8), (P,1)],
    &[(&H2, 7)]
    );

    println!("{}", lipid_15_0_18_1_d7_pc);
}