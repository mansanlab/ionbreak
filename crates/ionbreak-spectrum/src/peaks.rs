use super::spectrum::Peak;
use ionbreak_isotopes::atom::Atom;
use ionbreak_isotopes::molecule::Molecule;

// implements isoDalton from https://doi.org/10.1016/j.jasms.2007.05.016

fn merge_nearby_peaks(peaks: &mut Vec<Peak>) {
    let mut merged: Vec<Peak> = Vec::with_capacity(peaks.len());
    let mut i = 0;
    while i < peaks.len() {
        let anchor = peaks[i].amu;
        let epsilon = anchor / 1e15;
        let j = peaks[i..]
            .iter()
            .position(|s| (s.amu - anchor).abs() > epsilon)
            .map(|offset| i + offset)
            .unwrap_or(peaks.len());
        let group = &peaks[i..j];
        if group.len() == 1 {
            merged.push(group[0].clone());
        } else {
            let avg_amu = group.iter().map(|s| s.amu).sum::<f64>() / group.len() as f64;
            let total_prob = group.iter().map(|s| s.prob).sum::<f64>();
            merged.push(Peak { amu: avg_amu, prob: total_prob });
        }
        i = j;
    }
    *peaks = merged;
}

fn expand_peaks(
    peaks: &mut Vec<Peak>,
    isotopes: &[&'static Atom],
    next_peaks: &mut Vec<Peak>,
    max_peaks: usize,
) {
    next_peaks.clear();
    next_peaks.reserve(peaks.len() * isotopes.len());

    for peak in &*peaks {
        for &atom in isotopes {
            next_peaks.push(Peak {
                amu:  peak.amu + atom.amu,
                prob: peak.prob * atom.prob,
            });
        }
    }

    next_peaks.sort_unstable_by(|a, b| a.amu.partial_cmp(&b.amu).unwrap_or(std::cmp::Ordering::Equal));
    merge_nearby_peaks(next_peaks);
    next_peaks.sort_unstable_by(|a, b| b.prob.partial_cmp(&a.prob).unwrap_or(std::cmp::Ordering::Equal));
    next_peaks.truncate(max_peaks);

    std::mem::swap(peaks, next_peaks);
}

pub fn isotope_distribution(molecule: &Molecule<'_>, max_peaks: usize) -> Vec<Peak> {
    let mut peaks: Vec<Peak> = Vec::new(); // This is a spectrum
    let mut next_peaks: Vec<Peak> = Vec::new();

    let mut sorted_elements: Vec<(&'static [&'static Atom], usize)> = molecule
        .elements
        .iter()
        .map(|&(element, count)| (element.atoms, count))
        .collect();

    sorted_elements.sort_unstable_by(|(a_atoms, _), (b_atoms, _)| {
        a_atoms.len().cmp(&b_atoms.len()).then_with(|| {
            let a_avg = a_atoms.iter().map(|a| a.amu).sum::<f64>() / a_atoms.len() as f64;
            let b_avg = b_atoms.iter().map(|a| a.amu).sum::<f64>() / b_atoms.len() as f64;
            a_avg.partial_cmp(&b_avg).unwrap_or(std::cmp::Ordering::Equal)
        })
    });

    // seed from the first element group's isotopes
    let (seed_atoms, _seed_count) = sorted_elements[0];
    peaks.extend(seed_atoms.iter().map(|&a| Peak { amu: a.amu, prob: a.prob }));

    // expand remaining element groups (first group from repeat 1, rest from repeat 0)
    for (i, &(atoms, count)) in sorted_elements.iter().enumerate() {
        let start = if i == 0 { 1 } else { 0 };
        for _ in start..count {
            expand_peaks(&mut peaks, &atoms, &mut next_peaks, max_peaks);
        }
    }

    // apply fixed atoms as pure mass shifts (prob = 1.0, no branching)
    for &(atom, count) in molecule.atoms.iter() {
        let total_amu = atom.amu * count as f64;
        for peak in peaks.iter_mut() {
            peak.amu += total_amu;
        }
    }

    peaks
}