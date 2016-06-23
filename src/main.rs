mod player;
mod map;

fn main() {
    let map = map::Map::new(3);
    println!("{:?}", map);
}
