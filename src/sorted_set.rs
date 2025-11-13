#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SortedSet<T: Ord>(Vec<T>);
impl<T: Ord> SortedSet<T> {
    pub fn new() -> Self {
        SortedSet(Vec::new())
    }

    pub fn insert(&mut self, value: T) -> bool {
        match self.0.binary_search(&value) {
            Ok(_) => false, // already exists
            Err(pos) => {
                self.0.insert(pos, value);
                true
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.0.binary_search(value).is_ok()
    }

    pub fn remove(&mut self, value: &T) -> bool {
        match self.0.binary_search(value) {
            Ok(pos) => {
                self.0.remove(pos);
                true
            }
            Err(_) => false,
        }
    }
}

impl<T: Ord> From<Vec<T>> for SortedSet<T> {
    fn from(mut vec: Vec<T>) -> Self {
        vec.sort();
        vec.dedup();
        SortedSet(vec)
    }
}

impl<T: Ord + Clone> From<&[T]> for SortedSet<T> {
    fn from(slice: &[T]) -> Self {
        let mut vec = slice.to_vec();
        vec.sort();
        vec.dedup();
        SortedSet(vec)
    }
}

impl<T: Ord> Default for SortedSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
