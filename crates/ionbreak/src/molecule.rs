use crate::element::Element;

#[derive(Clone, Copy, Debug)]
pub struct Molecule {
    // Collection of elements
    pub elements: &'static [Element]
}