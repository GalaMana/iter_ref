//! Traits for iterating over referenced data without consuming the iterator.
//! 
//! I was tired of the iter() function not being a trait. So I made it one!
//! 
//! 

pub mod prelude;

/// Trait for iterating over references without consuming the iterator.
pub trait IterRef: IntoIterator {
    /// Iterates over the items as immutable references.
    /// 
    /// A 'reference' is a type that points to a different spot in memory where the actual data is.
    /// Being 'immutable' means that the reference can't change the underlying data.
    /// If you want to change the underlying data then use iter_mut()
    fn iter_better(&self) -> impl Iterator<Item = &Self::Item>;
}
pub trait IterRefMut: IterRef {
    /// Iterates over the items as mutable references.
    /// 
    /// A 'reference' is a type that points to a different spot in memory where the actual data is.
    /// Being 'mutable' means that the reference can change the underlying data.
    /// 
    /// If you don't need to change the underlying data then prefer using 'iter()'
    fn iter_better_mut(&mut self) -> impl Iterator<Item = &mut Self::Item>;
}

impl<T> IterRef for T
where T: IntoIterator<>,
for<'a> &'a T: IntoIterator<Item = &'a T::Item>
{
    //type Item = <T as IntoIterator>::Item;

    fn iter_better(&self) -> impl Iterator<Item = &Self::Item> {
        self.into_iter()
    }
}

impl<T> IterRefMut for T
where T: IterRef,
for<'a> &'a mut T: IntoIterator<Item = &'a mut <T as IntoIterator>::Item>
{
    fn iter_better_mut(&mut self) -> impl Iterator<Item = &mut Self::Item> {
        self.into_iter()
    }
}