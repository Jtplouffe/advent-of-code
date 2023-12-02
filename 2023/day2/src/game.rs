use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
}

impl From<&str> for CubeColor {
    fn from(value: &str) -> Self {
        match value {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => unreachable!(),
        }
    }
}

pub struct Game {
    pub id: usize,
    pub minimum_colored_cubes: HashMap<CubeColor, usize>,
}

impl Game {
    pub fn are_cubes_possible(&self, cubes: &[(CubeColor, usize)]) -> bool {
        for (color, quantity) in cubes {
            match self.minimum_colored_cubes.get(color) {
                Some(minimun_quantity) => {
                    if minimun_quantity > quantity {
                        return false;
                    }
                }
                None => return false,
            }
        }

        true
    }

    pub fn minimun_colored_cubes_qauntity_power(&self) -> usize {
        self.minimum_colored_cubes.values().product()
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game, draws) = value.split_once(": ").unwrap();

        let id: usize = game.split_whitespace().nth(1).unwrap().parse().unwrap();

        let draws = draws.replace(';', ",");
        let cubes = draws.split(", ").map(|cube| {
            let (count_str, color_str) = cube.split_once(' ').unwrap();
            (
                CubeColor::from(color_str),
                count_str.parse::<usize>().unwrap(),
            )
        });

        let mut minimum_colored_cubes = HashMap::new();
        for (color, quantity) in cubes {
            let minimum = minimum_colored_cubes.entry(color).or_insert(0);
            if *minimum < quantity {
                *minimum = quantity;
            }
        }

        Self {
            id,
            minimum_colored_cubes,
        }
    }
}
