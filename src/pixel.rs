use rand::{seq::SliceRandom, Rng};

use super::grid::ImplementedGrid;

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum PixelChangeResult {
    Unchanged,
    Updated,
    Invalid,
}

/// This trait needs to be implemented for all types used in microwfc to allow microwfc to know the superposition.
/// A value *can* be added multiple times.
pub trait PossibleValues: Sized + Clone {
    fn get_possible_values() -> Vec<Self>;
}

/// This struct represents a n-dimensional "pixel".
#[derive(Clone)]
pub struct Pixel<T: Clone> {
    pub(crate) possible_values: Vec<T>,
    pub determined_value: Option<T>,
}

impl<T: PossibleValues> Default for Pixel<T> {
    fn default() -> Self {
        Self {
            possible_values: T::get_possible_values(),
            determined_value: None,
        }
    }
}

impl<T: Clone> Pixel<T> {
    pub fn new(item: T) -> Pixel<T> {
        Pixel {
            possible_values: vec![item.clone()],
            determined_value: Some(item),
        }
    }

    pub(crate) fn recalc<L, SL, G, F>(
        &mut self,
        grid: &G,
        location: L,
        test: F,
        randomize: Option<&mut impl Rng>,
    ) -> PixelChangeResult
    where
        L: Copy,
        SL: Copy,
        G: ImplementedGrid<L, T, SL>,
        F: Fn(&G, L, &T) -> bool,
    {
        let mut r = PixelChangeResult::Unchanged;
        let len = self.possible_values.len();
        for (i, val) in self.possible_values.clone().iter().rev().enumerate() {
            if !test(grid, location, val) {
                self.possible_values.remove(len - i - 1);
                r = PixelChangeResult::Updated;
            }
        }
        if self.possible_values.is_empty() {
            return PixelChangeResult::Invalid; // reset and re-randomize
        }
        if self.determined_value.is_some() {
            return PixelChangeResult::Unchanged;
        }
        if self.possible_values.len() == 1 {
            self.determined_value = Some(self.possible_values[0].clone());
            return PixelChangeResult::Updated;
        }
        if let Some(rng) = randomize {
            // SAFETY: The following is safe because the list is known to be non-empty.
            self.determined_value = Some(self.possible_values.choose(rng).unwrap().clone());
            self.possible_values = vec![self.determined_value.as_ref().unwrap().clone()];
            r = PixelChangeResult::Updated;
        }
        r
    }
}
