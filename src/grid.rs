use micro_ndarray::Array;
use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};

use crate::{pixel::PixelChangeResult, PossibleValues, Unique};

use super::pixel::Pixel;

/// Trait representing an error during Grid initialization. The only possible error is currently SizeMustNotBeZero.
#[derive(Debug)]
pub enum SizeErr {
    SizeMustNotBeZero,
}

/// A microwfc grid.
pub struct Grid<T: PossibleValues, const D: usize> {
    pub(crate) size: [usize; D],
    pub(crate) data: Array<Pixel<T>, D>,
}

impl<T: PossibleValues, const D: usize> Grid<T, D> {
    /// Constructs a new Grid using the n-dimensional size.
    pub fn new(size: [usize; D]) -> Result<Self, SizeErr> {
        if size.iter().any(|x| *x == 0) {
            return Err(SizeErr::SizeMustNotBeZero);
        }
        Ok(Self {
            size,
            data: Array::new(size),
        })
    }

    /// Returns the n-dimensional size of the Grid.
    /// In all default implementations, this returns a n-tuple where n is the dimensionality of the Grid.
    pub fn size(&self) -> [usize; D] {
        self.size
    }

    /// Clones and returns a Pixel from the Grid.
    pub fn get_item(&self, location: [usize; D]) -> Pixel<T> {
        self.data[location].clone()
    }

    /// Sets a Pixel in the Grid.
    pub fn set_item(&mut self, location: [usize; D], item: Pixel<T>) {
        self.data[location] = item;
    }

    /// Returns unidirectional neighbors, meaning only neighbord with one common face.
    /// This means the corners will not be returned.
    pub fn unidirectional_neighbors(&self, location: [usize; D]) -> Vec<([usize; D], Pixel<T>)> {
        self.neighbors(location, 1)
            .into_iter()
            .filter(|neighbor| {
                neighbor
                    .0
                    .iter()
                    .enumerate()
                    .filter(|(i, x)| location[*i] - **x != 0)
                    .count()
                    == 1
            })
            .collect()
    }
    /// Returns all neighbord, including ones touching only at a single point.
    /// This does return corners.
    pub fn neighbors(&self, location: [usize; D], distance: usize) -> Vec<([usize; D], Pixel<T>)> {
        let mut r = Vec::new();
        let start_location: [usize; D] = location
            .into_iter()
            .map(|x| if x < distance { x } else { x - distance })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let mut loc = start_location;
        loop {
            r.push((loc, self.get_item(loc)));
            for n in 0..D {
                loc[n] += 1;
                if loc[n] > location[n] + distance || loc[n] == self.size[n] {
                    loc[n] = start_location[n];
                } else {
                    break;
                }
            }
            if loc == start_location {
                // will reset to [0; D] when end is reached, but will NOT reach that before as it is incremented first
                break;
            }
        }
        r
    }

    /// Checks if a location is inside the Grid, then returns its Grid coordinates.
    pub fn check_loc(&self, location: [i128; D]) -> Option<[usize; D]> {
        for (i, dimensionality) in self.size.iter().enumerate() {
            if location[i] < 0 || location[i] >= *dimensionality as i128 {
                return None;
            }
        }
        Some(location.map(|x| x as usize))
    }

    /// Checks if the Grid is valid
    pub fn check_validity<F>(&mut self, test: F) -> Result<(), [usize; D]>
    where
        F: Fn(&Self, [usize; D], &T) -> bool,
    {
        let mut data = self.data.clone();
        for (loc, pixel) in data.iter_mut() {
            if pixel.determined_value.is_some() {
                continue;
            }
            if let PixelChangeResult::Invalid =
                pixel.recalc(self, loc, &test, None::<&mut rand::rngs::mock::StepRng>)
            {
                return Err(loc);
            }
        }
        self.data = data;
        Ok(())
    }

    fn update<F: Fn(&Self, [usize; D], &T) -> bool>(
        &mut self,
        to_update: &mut Vec<([usize; D], Pixel<T>)>,
        (location, mut pixel): ([usize; D], Pixel<T>),
        test: &F,
        effect_distance: usize,
        rng: &mut impl Rng,
        should_collapse: bool,
    ) -> PixelChangeResult {
        let result = pixel.recalc(
            self,
            location,
            test,
            if should_collapse { Some(rng) } else { None },
        );
        match result {
            PixelChangeResult::Invalid => {
                return PixelChangeResult::Invalid;
            }
            PixelChangeResult::Updated => {
                self.data[location] = pixel;
                let mut to_add = self.neighbors(location, effect_distance);
                to_add.shuffle(rng);
                to_update.append(&mut to_add);
            }
            PixelChangeResult::Unchanged => return result,
        }
        result
    }

    /// Collapses a single Pixel and updates neighbors. This will return false if the Grid is invalid.
    /// Please note that this function is not very useful, and you should use wfc instead
    pub fn collapse<F, R>(
        &mut self,
        test: F,
        effect_distance: usize,
        rng: &mut R,
        item: ([usize; D], Pixel<T>),
    ) -> Result<(), [usize; D]>
    where
        F: Fn(&Self, [usize; D], &T) -> bool,
        R: Rng,
    {
        let mut to_update = vec![item];
        let mut i = 0;
        while !to_update.is_empty() {
            let item = to_update.remove(0);
            let r = self.update(
                &mut to_update,
                (item.0, item.1.clone()),
                &test,
                effect_distance,
                rng,
                i == 0,
            );
            if r == PixelChangeResult::Invalid {
                return Err(item.0);
            }
            i += 1;
        }
        Ok(())
    }

    /// Performs the wave-function-collapse algorithm on the Grid.
    /// This returns if the algorithm was successful, and the state of the Grid is not guaranteed
    /// to be valid if it returns false, but there is never unsafety in reading from the Grid.
    ///
    /// The chance parameter determines the likelyhood of a random collapse happening anywhere on the
    /// grid. In some strict environments, this can cause unsolvable grids.
    pub fn wfc<F, R>(
        &mut self,
        test: F,
        effect_distance: usize,
        rng: &mut R,
        chance: f32,
        on_update: impl Fn(&Self),
    ) -> Result<(), [usize; D]>
    where
        F: Fn(&Self, [usize; D], &T) -> bool,
        R: Rng,
    {
        self.check_validity(&test)?;
        loop {
            let backup = self.data.clone();

            // Get all items that haven't been determined yet
            let to_update: Vec<_> = self
                .data
                .iter()
                .filter(|x| x.1.determined_value.is_none())
                .collect();

            if to_update.is_empty() {
                // We're done
                break;
            }

            // Get a random pixel with minimal entropy and collapse it
            let min = to_update
                .iter()
                .min_by_key(|x| x.1.possible_values.unique().len())
                .unwrap() // SAFETY: This is safe because the list is known to be non-empty.
                .1
                .possible_values
                .unique()
                .len();
            let to_update = if rng.gen::<f32>() > chance {
                to_update
                    .into_iter()
                    .filter(|x| x.1.possible_values.unique().len() == min)
                    .map(|x| (x.0, x.1.clone()))
                    .choose(rng)
                    .unwrap() // SAFETY: This is safe because the list is known to be non-empty.
            } else {
                to_update
                    .into_iter()
                    .choose(rng)
                    .map(|x| (x.0, x.1.clone()))
                    .unwrap() // SAFETY: This is safe because the list is known to be non-empty.
            };

            let loc = to_update.0;

            // Now collapse the Pixel
            if self
                .collapse(&test, effect_distance, rng, to_update)
                .is_err()
            {
                self.data = backup;
                self.data[loc] = Pixel::default();
            }

            on_update(self);
        }
        Ok(())
    }
}
