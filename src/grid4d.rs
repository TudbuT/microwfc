use crate::{Grid, ImplementedGrid, Pixel, PossibleValues};

impl<T: PossibleValues> ImplementedGrid<T, 4> for Grid<T, 4> {
    fn unidirectional_neighbors(&self, location: [usize; 4]) -> Vec<Pixel<T>> {
        let mut v = Vec::new();
        if location[0] > 0 {
            v.push(self.get_item([location[0] - 1, location[1], location[2], location[3]]));
        }
        if location[1] > 0 {
            v.push(self.get_item([location[0], location[1] - 1, location[2], location[3]]));
        }
        if location[2] > 0 {
            v.push(self.get_item([location[0], location[1], location[2] - 1, location[3]]));
        }
        if location[3] > 0 {
            v.push(self.get_item([location[0], location[1], location[2], location[3] - 1]));
        }
        if location[3] < self.size[3] - 1 {
            v.push(self.get_item([location[0], location[1], location[2], location[3] + 1]));
        }
        if location[2] < self.size[2] - 1 {
            v.push(self.get_item([location[0], location[1], location[2] + 1, location[3]]));
        }
        if location[1] < self.size[1] - 1 {
            v.push(self.get_item([location[0], location[1] + 1, location[2], location[3]]));
        }
        if location[0] < self.size[0] - 1 {
            v.push(self.get_item([location[0] + 1, location[1], location[2], location[3]]));
        }
        v
    }

    fn neighbors(&self, location: [usize; 4], distance: usize) -> Vec<([usize; 4], Pixel<T>)> {
        let mut v = Vec::new();
        for a in 0..=(distance * 2) {
            for z in 0..=(distance * 2) {
                for y in 0..=(distance * 2) {
                    for x in 0..=(distance * 2) {
                        let location = [
                            location[0] as i128 + x as i128 - distance as i128,
                            location[1] as i128 + y as i128 - distance as i128,
                            location[2] as i128 + z as i128 - distance as i128,
                            location[3] as i128 + a as i128 - distance as i128,
                        ];
                        if let Some(location) = self.check_loc(location) {
                            v.push((location, self.get_item(location)));
                        }
                    }
                }
            }
        }
        v
    }
}
