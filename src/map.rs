#[derive(Debug)]
pub struct WallGrid {
    grid: Vec<u8>,
}

#[derive(Debug)]
pub struct Map {
    size: usize,
    wall_grid: WallGrid,
}

impl Map {
    pub fn new(size: usize) -> Map {
        let base_size = vec![0; size * size];

        Map {
            size: size,
            wall_grid: WallGrid {
                grid: base_size.clone(),
            }
        }
    }
}
