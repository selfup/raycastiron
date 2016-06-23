extern crate rand;

use self::rand::Rng;

#[derive(Debug)]
pub struct WallGrid {
    grid: Vec<u8>,
}

#[derive(Debug)]
pub struct Map {
    size: u8,
    wall_grid: WallGrid,
}

impl Map {
    pub fn new(size: u8) -> Map {
        let squared = usize::from(size * size);
        let base_size = vec![0; squared];

        Map {
            size: size,
            wall_grid: WallGrid {
                grid: base_size.clone(),
            },
        }
    }

    pub fn get(self, x: f32, y: f32) -> i32 {
        let x = x.floor();
        let y = y.floor();

        let current_size = f32::from(self.size);

        if x < 0.0 || x > current_size - 1.0 || y < 0.0 || y > current_size - 1.0 {
            let return_val: i32 = -1;
            return_val
        } else {
            let index = y * current_size + x;
            println!("{:?}", index);
            // println!("{:?}", self.wall_grid.grid.position_elem(index));
            // let return_val = self.wall_grid.grid[index];
            123
        }
    }

    pub fn randomize(&mut self) -> &mut Map {
        let map_size = self.size * self.size;
        for i in 0..map_size {
            let num = rand::thread_rng().gen_range(0, 2);
            let i = usize::from(i);
            self.wall_grid.grid[i] = num;
        };
        self
    }
}
