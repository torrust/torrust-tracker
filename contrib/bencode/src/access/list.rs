use std::ops::{Index, IndexMut};

/// Trait for working with generic list data structures.
pub trait BListAccess<V> {
    /// Get a list element at the given index.
    fn get(&self, index: usize) -> Option<&V>;

    /// Get a mutable list element at the given index.
    fn get_mut(&mut self, index: usize) -> Option<&mut V>;

    /// Remove a list element at the given index.
    fn remove(&mut self, index: usize) -> Option<V>;

    /// Insert a list element at the given index.
    fn insert(&mut self, index: usize, item: V);

    /// Push an element to the back of the list.
    fn push(&mut self, item: V);

    /// Get the length of the list.
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
}

impl<'a, V: 'a> Index<usize> for &'a dyn BListAccess<V> {
    type Output = V;

    fn index(&self, index: usize) -> &V {
        self.get(index).unwrap()
    }
}

impl<'a, V: 'a> Index<usize> for &'a mut dyn BListAccess<V> {
    type Output = V;

    fn index(&self, index: usize) -> &V {
        self.get(index).unwrap()
    }
}

impl<'a, V: 'a> IndexMut<usize> for &'a mut dyn BListAccess<V> {
    fn index_mut(&mut self, index: usize) -> &mut V {
        self.get_mut(index).unwrap()
    }
}

impl<'a, V: 'a> IntoIterator for &'a dyn BListAccess<V> {
    type Item = &'a V;
    type IntoIter = BListIter<'a, V>;

    fn into_iter(self) -> BListIter<'a, V> {
        BListIter { index: 0, access: self }
    }
}

pub struct BListIter<'a, V> {
    index: usize,
    access: &'a dyn BListAccess<V>,
}

impl<'a, V> Iterator for BListIter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        let opt_next = self.access.get(self.index);

        if opt_next.is_some() {
            self.index += 1;
        }

        opt_next
    }
}

impl<V> BListAccess<V> for Vec<V> {
    fn get(&self, index: usize) -> Option<&V> {
        self[..].get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut V> {
        self[..].get_mut(index)
    }

    fn remove(&mut self, index: usize) -> Option<V> {
        if index >= self[..].len() {
            None
        } else {
            Some(Vec::remove(self, index))
        }
    }

    fn insert(&mut self, index: usize, item: V) {
        Vec::insert(self, index, item);
    }

    fn push(&mut self, item: V) {
        Vec::push(self, item);
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}
