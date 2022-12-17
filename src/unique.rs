/// Trait to extract unique items from collections.
pub trait Unique {
    /// Returns the same collection, but with all duplicates removed
    fn unique(&self) -> Self;
}

impl<T: Clone + PartialEq> Unique for Vec<T> {
    fn unique(&self) -> Self {
        let mut r = Vec::new();
        for item in self {
            if !r.contains(item) {
                r.push(item.clone());
            }
        }
        r
    }
}
