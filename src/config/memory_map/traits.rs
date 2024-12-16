pub trait Contains<T> {
    fn contains(&self, value: &T) -> bool;
}

pub trait Overlapping<T> {
    fn overlapping(&self, value: &T) -> bool;
}
