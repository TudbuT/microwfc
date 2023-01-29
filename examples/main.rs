use std::{thread, time::Duration};

use microwfc::*;

use rand::thread_rng;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Tile {
    Water,
    Dirt,
    Grass,
}

impl From<Tile> for String {
    fn from(val: Tile) -> Self {
        // See ANSI escape codes for 8-bit colors:
        // https://en.wikipedia.org/wiki/ANSI_escape_sequence#8-bit
        match val {
            Tile::Water => "\x1b[38;5;21m██\x1b[0m".to_string(),
            Tile::Dirt => "\x1b[38;5;94m██\x1b[0m".to_string(),
            Tile::Grass => "\x1b[38;5;28m██\x1b[0m".to_string(),
        }
    }
}

impl PossibleValues for Tile {
    fn get_possible_values() -> Vec<(Self, f32)> {
        vec![(Self::Grass, 4f32), (Self::Dirt, 1f32), (Self::Water, 3f32)]
    }
}

fn main() {
    let mut rng = thread_rng();
    // Make a new 30-by-30 grid.
    let mut grid: Grid<Tile, 2> = Grid::new([30, 30]).unwrap();
    loop {
        let r = grid.wfc(
            |g, loc, me, probability| {
                // We use !any(|x| ...) to get none(|x| ...) functionality
                match *me {
                    // Disallow water next to grass
                    Tile::Water => (
                        !g.unidirectional_neighbors(loc).iter().any(|x| {
                            x.1.determined_value
                                .as_ref()
                                .map(|x| *x == Tile::Grass)
                                .unwrap_or(false) // Allow unsolved pixels
                        }),
                        probability,
                    ),
                    // Dirt is always allowed
                    Tile::Dirt => (true, probability),
                    // Disallow grass next to water
                    Tile::Grass => (
                        !g.unidirectional_neighbors(loc).iter().any(|x| {
                            x.1.determined_value
                                .as_ref()
                                .map(|x| *x == Tile::Water)
                                .unwrap_or(false)
                        }),
                        probability,
                    ),
                }
            },
            1,
            &mut rng,
            0.05,
            |grid| {
                // Clear the screen
                println!("\x1b[H\x1b[2J\x1b[3J");

                let mut s = String::new();
                for y in 0..grid.size()[0] {
                    s += "\n";
                    for x in 0..grid.size()[1] {
                        if let Some(x) = grid.get_item([x, y]).determined_value {
                            s += &String::from(x);
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
