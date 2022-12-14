use rand::Rng;

use super::pixel::Pixel;

/// Trait representing an error during Grid initialization. The only possible error is currently SizeMustNotBeZero.
#[derive(Debug)]
pub enum SizeErr {
    SizeMustNotBeZero,
}

/// Trait representing an implemented Grid, which is implemented for 2D and 3D grids by default.
pub trait ImplementedGrid<L, T: Clone, SL>: Sized {
    /// Constructs a new Grid using the n-dimensional size.
    fn new(size: L) -> Result<Self, SizeErr>;
    /// Clones and returns a Pixel from the Grid.
    fn get_item(&self, location: L) -> Pixel<T>;
    /// Sets a Pixel in the Grid.
    fn set_item(&mut self, location: L, item: Pixel<T>);

    /// Returns unidirectional neighbors, meaning only neighbord with one common face.
    /// This means the corners will not be returned.
    fn unidirectional_neighbors(&self, location: L) -> Vec<Pixel<T>>;
    /// Returns all neighbord, including ones touching only at a single point.
    /// This does return corners.
    fn neighbors(&self, location: L, distance: usize) -> Vec<(L, Pixel<T>)>;

    /// Checks if a location is inside the Grid, then returns its Grid coordinates.
    fn check_loc(&self, location: SL) -> Option<L>;

    /// Checks if the Grid is valid
    fn check_validity<F>(&mut self, test: F) -> Result<(), L>
    where
        F: Fn(&Self, L, &T) -> bool;

    /// Collapses a single Pixel and updates neighbors. This will return false if the Grid is invalid.
    /// Please note that this function is not very useful, and you should use wfc instead
    fn collapse<F, R>(
        &mut self,
        test: F,
        effect_distance: usize,
        rng: &mut R,
        data: (L, Pixel<T>),
    ) -> Result<(), L>
    where
        F: Fn(&Self, L, &T) -> bool,
        R: Rng;

    /// Performs the wave-function-collapse algorithm on the Grid.
    /// This returns if the algorithm was successful, and the state of the Grid is not guaranteed
    /// to be valid if it returns false, but there is never unsafety in reading from the Grid.
    fn wfc<F, R>(&mut self, test: F, effect_distance: usize, rng: &mut R) -> Result<(), L>
    where
        F: Fn(&Self, L, &T) -> bool,
        R: Rng;
}

/// A microwfc grid, which can exist in all dimensions, but is only implemented for 2D and 3D.
pub struct Grid<L: Clone, V: Clone> {
    pub(crate) size: L,
    pub(crate) data: V,
}

impl<L: Clone, V: Clone> Grid<L, V> {
    /// Returns the n-dimensional size of the Grid.
    /// In all default implementations, this returns a n-tuple where n is the dimensionality of the Grid.
    pub fn size(&self) -> L {
        self.size.clone()
    }
}
