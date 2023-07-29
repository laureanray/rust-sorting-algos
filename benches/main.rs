use orst::*;
use std::sync::{atomic::{AtomicUsize, Ordering::AcqRel}, Arc};

struct SortEvaluator<T> {
    t: T,
    cmps: Arc<AtomicUsize>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.fetch_add(1, AcqRel);
        self.t == other.t
    }
}

impl<T: Eq> Eq for SortEvaluator<T> { }

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmps.fetch_add(1, AcqRel);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.t.cmp(&other.t)
    }
}

fn main() {


}
