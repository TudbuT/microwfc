use std::{thread, time::Duration};

use microwfc::*;

use rand::thread_rng;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Test {
    Stone,
    Dirt,
    Grass,
}

impl PossibleValues for Test {
    fn get_possible_values() -> Vec<(Self, f32)> {
        vec![(Self::Grass, 4f32), (Self::Dirt, 1f32), (Self::Stone, 3f32)]
    }
}

fn main() {
    let mut rng = thread_rng();
    // Make a new 30-by-30 grid.
    let mut grid: Grid<Test, 2> = Grid::new([30, 30]).unwrap();
    loop {
        let r = grid.wfc(
            |g, loc, me| {
                // We use !any(|x| ...) to get none(|x| ...) functionality
                match *me {
                    // Disallow stone next to grass
                    Test::Stone => !g.unidirectional_neighbors(loc).iter().any(|x| {
                        x.1.determined_value
                            .as_ref()
                            .map(|x| *x == Test::Grass)
                            .unwrap_or(false) // Allow unsolved pixels
                    }),
                    // Dirt is always allowed
                    Test::Dirt => true,
                    // Disallow grass next to stone
                    Test::Grass => !g.unidirectional_neighbors(loc).iter().any(|x| {
                        x.1.determined_value
                            .as_ref()
                            .map(|x| *x == Test::Stone)
                            .unwrap_or(false)
                    }),
                }
            },
            1,
            &mut rng,
            0.05,
            |grid| {
                let mut s = String::new();
                for y in 0..grid.size()[0] {
                    s += "\n";
                    for x in 0..grid.size()[1] {
                        if let Some(x) = grid.get_item([x, y]).determined_value {
                            s += match x {
                                Test::Stone => "##",
                                Test::Dirt => "YY",
                                Test::Grass => "//",
                            };
                        } else {
                            s += "  ";
                        }
                    }
                }
                println!("{}", s);
                thread::sleep(Duration::from_millis(10));
            },
        );
        if r.is_ok() {
            break;
        } else {
            println!("fuck");
        }
    }
}
