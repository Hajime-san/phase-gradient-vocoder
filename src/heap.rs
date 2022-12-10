use std::cmp::Ordering;

#[derive(Debug)]
pub struct MaxHeap {
    pub magnitude: f64,
    pub frequency_index: usize,
    pub frame: usize
}

impl PartialEq for MaxHeap {
    fn eq(&self, other: &Self) -> bool {
        self.magnitude == other.magnitude
    }
}

impl Eq for MaxHeap {}

impl Ord for MaxHeap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for MaxHeap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.magnitude.partial_cmp(&other.magnitude)
    }
}
