use rand::Rng;

use super::pixel::Pixel;

#[derive(Debug)]
pub enum SizeErr {
    SizeMustNotBeZero,
}

pub trait ImplementedGrid<L, T: Clone, SL>: Sized {
    fn new(size: L) -> Result<Self, SizeErr>;
    fn get_item(&self, location: L) -> Pixel<T>;
    fn set_item(&mut self, location: L, item: Pixel<T>);

    fn unidirectional_neighbors(&self, location: L) -> Vec<Pixel<T>>;
    fn neighbors(&self, location: L) -> Vec<Pixel<T>>;

    fn check_loc(&self, location: SL) -> Option<L>;

    fn wfc<F, R>(&mut self, test: F, effect_distance: usize, rng: &mut R) -> bool
    where
        F: Fn(&Self, L, &T) -> bool,
        R: Rng;
}

pub struct Grid<L: Clone, V: Clone> {
    pub(crate) size: L,
    pub(crate) data: V,
}

impl<L: Clone, V: Clone> Grid<L, V> {
    pub fn size(&self) -> L {
        self.size.clone()
    }
}
