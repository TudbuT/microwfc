use std::fmt::Debug;

use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};

use crate::{
    grid::SizeErr, pixel::PixelChangeResult, Grid, ImplementedGrid, Pixel, PossibleValues,
};

pub type Vec3i = (usize, usize, usize);

impl<T: PossibleValues + Debug> Grid<Vec3i, Vec<Vec<Vec<Pixel<T>>>>> {
    fn update<F: Fn(&Self, Vec3i, &T) -> bool>(
        &mut self,
        to_update: &mut Vec<(Vec3i, Pixel<T>)>,
        ((x, y, z), mut pixel): (Vec3i, Pixel<T>),
        test: &F,
        effect_distance: usize,
        rng: &mut impl Rng,
        should_collapse: bool,
    ) -> PixelChangeResult {
        let result = pixel.recalc(
            self,
            (x, y, z),
            &test,
            if should_collapse { Some(rng) } else { None },
        );
        match result {
            PixelChangeResult::Invalid => {
                return PixelChangeResult::Invalid;
            }
            PixelChangeResult::Updated => {
                self.data[x][y][z] = pixel;
                let mut to_add = self.neighbors((x, y, z), effect_distance);
                to_add.shuffle(rng);
                to_update.append(&mut to_add);
            }
            PixelChangeResult::Unchanged => return result,
        }
        result
    }
}

impl<T: PossibleValues + Debug> ImplementedGrid<Vec3i, T, (i128, i128, i128)>
    for Grid<Vec3i, Vec<Vec<Vec<Pixel<T>>>>>
{
    fn new(size: Vec3i) -> Result<Self, SizeErr> {
        if size.0 == 0 || size.1 == 0 || size.2 == 0 {
            return Err(SizeErr::SizeMustNotBeZero);
        }
        Ok(Self {
            size,
            data: vec![vec![vec![Pixel::default(); size.2]; size.1]; size.0],
        })
    }

    fn get_item(&self, location: Vec3i) -> Pixel<T> {
        self.data[location.0][location.1][location.2].clone()
    }

    fn set_item(&mut self, location: Vec3i, item: Pixel<T>) {
        self.data[location.0][location.1][location.2] = item;
    }

    fn unidirectional_neighbors(&self, location: Vec3i) -> Vec<Pixel<T>> {
        let mut v = Vec::new();
        if location.0 > 0 {
            v.push(self.get_item((location.0 - 1, location.1, location.2)));
        }
        if location.1 > 0 {
            v.push(self.get_item((location.0, location.1 - 1, location.2)));
        }
        if location.2 > 0 {
            v.push(self.get_item((location.0, location.1, location.2 - 1)));
        }
        if location.2 < self.size.2 - 1 {
            v.push(self.get_item((location.0, location.1, location.2 + 1)));
        }
        if location.1 < self.size.1 - 1 {
            v.push(self.get_item((location.0, location.1 + 1, location.2)));
        }
        if location.0 < self.size.0 - 1 {
            v.push(self.get_item((location.0 + 1, location.1, location.2)));
        }
        v
    }

    fn neighbors(&self, location: Vec3i, distance: usize) -> Vec<(Vec3i, Pixel<T>)> {
        let mut v = Vec::new();
        for z in 0..=(distance * 2) {
            for y in 0..=(distance * 2) {
                for x in 0..=(distance * 2) {
                    let location = (
                        location.0 as i128 + x as i128 - distance as i128,
                        location.1 as i128 + y as i128 - distance as i128,
                        location.2 as i128 + z as i128 - distance as i128,
                    );
                    if let Some(location) = self.check_loc(location) {
                        v.push((location, self.get_item(location)));
                    }
                }
            }
        }
        v
    }

    fn check_loc(&self, location: (i128, i128, i128)) -> Option<Vec3i> {
        if location.0 < 0
            || location.1 < 0
            || location.2 < 0
            || location.0 >= self.size.0 as i128
            || location.1 >= self.size.1 as i128
            || location.2 >= self.size.2 as i128
        {
            None
        } else {
            Some((
                location.0 as usize,
                location.1 as usize,
                location.2 as usize,
            ))
        }
    }

    fn check_validity<F>(&mut self, test: F) -> Result<(), Vec3i>
    where
        F: Fn(&Self, Vec3i, &T) -> bool,
    {
        let mut data = self.data.clone();
        for (z, zv) in data.iter_mut().enumerate() {
            for (y, yv) in zv.iter_mut().enumerate() {
                for (x, pixel) in yv.iter_mut().enumerate() {
                    if pixel.determined_value.is_some() {
                        continue;
                    }
                    if let PixelChangeResult::Invalid = pixel.recalc(
                        self,
                        (x, y, z),
                        &test,
                        None::<&mut rand::rngs::mock::StepRng>,
                    ) {
                        return Err((x, y, z));
                    }
                }
            }
        }
        self.data = data;
        Ok(())
    }

    fn collapse<F, R>(
        &mut self,
        test: F,
        effect_distance: usize,
        rng: &mut R,
        item: (Vec3i, Pixel<T>),
    ) -> Result<(), Vec3i>
    where
        F: Fn(&Self, Vec3i, &T) -> bool,
        R: Rng,
    {
        let mut to_update = vec![item];
        let mut i = 0;
        while !to_update.is_empty() {
            let item = to_update.remove(0);
            let r = self.update(
                &mut to_update,
                (item.0, item.1),
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

    fn wfc<F, R>(
        &mut self,
        test: F,
        effect_distance: usize,
        rng: &mut R,
        chance: f32,
    ) -> Result<(), Vec3i>
    where
        F: Fn(&Self, Vec3i, &T) -> bool,
        R: Rng,
    {
        self.check_validity(&test)?;
        loop {
            let backup = self.data.clone();

            // Get all items that haven't been determined yet
            let to_update: Vec<_> = self
                .data
                .iter()
                .enumerate()
                .flat_map(|(x, v)| {
                    v.iter().enumerate().flat_map(move |(y, v)| {
                        v.iter()
                            .enumerate()
                            .filter(|(_, pixel)| pixel.determined_value.is_none())
                            .map(move |(z, pixel)| ((x, y, z), pixel.clone()))
                    })
                })
                .collect();

            if to_update.is_empty() {
                // We're done
                break;
            }

            // Get a random pixel with minimal entropy and collapse it
            let min = to_update
                .iter()
                .min_by(|a, b| a.1.possible_values.len().cmp(&b.1.possible_values.len()))
                .unwrap() // SAFETY: This is safe because the list is known to be non-empty.
                .1
                .possible_values
                .len();
            let to_update = if rng.gen::<f32>() > chance {
                to_update
                    .into_iter()
                    .filter(|x| x.1.possible_values.len() == min)
                    .choose(rng)
                    .unwrap() // SAFETY: This is safe because the list is known to be non-empty.
            } else {
                to_update.into_iter().choose(rng).unwrap() // SAFETY: This is safe because the list is known to be non-empty.
            };

            // Now collapse the Pixel
            if self
                .collapse(&test, effect_distance, rng, to_update)
                .is_err()
            {
                self.data = backup;
            }
        }
        Ok(())
    }
}
