use ionbreak_isotopes::isotope::{PROTON};

#[derive(Clone)]
pub struct Peak {
    pub amu: f64,
    pub prob: f64,
}

#[derive(Default)]
pub struct Spectrum {
    pub peaks: Vec<Peak>,
    pub z: isize
}

impl Spectrum {
    pub fn new(peaks: Vec<Peak>, z: isize) -> Self {
        Self { peaks, z }
    }

    pub fn mz(&self) -> Vec<Peak> {
        let z: f64 = self.z as f64;

        self.peaks.iter().map(|p| {
            Peak {
                amu: (p.amu + (z * PROTON.amu)) / z.abs(), 
                prob: p.prob 
            }
        }).collect()
    }
}