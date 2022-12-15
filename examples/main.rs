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
    fn get_possible_values() -> Vec<Self> {
        vec![
            // We add some values more to increase their probability, and we add dirt much less to decrease
            // probability of a jump to stone. This means we will get large-ish areas of grass and stone, with dirt borders.
            Self::Grass,
            Self::Grass,
            Self::Grass,
            Self::Grass,
            Self::Dirt,
            Self::Stone,
            Self::Stone,
            Self::Stone,
        ]
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
                        x.determined_value
                            .as_ref()
                            .map(|x| *x == Test::Grass)
                            .unwrap_or(false) // Allow unsolved pixels
                    }),
                    // Dirt is always allowed
                    Test::Dirt => true,
                    // Disallow grass next to stone
                    Test::Grass => !g.unidirectional_neighbors(loc).iter().any(|x| {
                        x.determined_value
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
                    s += "\n";
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
