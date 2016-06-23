mod player;
mod map;

fn main() {
    let mut map = map::Map::new(10);
    println!("{:?}", map);
    println!("{:?}", map.randomize());
    // println!("{:?}", map.get(2.0, 2.0));
}


pub fn ray(origin: u8) {

}

// function ray(origin) {
//   var stepX = step(sin, cos, origin.x, origin.y);
//   var stepY = step(cos, sin, origin.y, origin.x, true);
//   var nextStep = stepX.length2 < stepY.length2
//     ? inspect(stepX, 1, 0, origin.distance, stepX.y)
//     : inspect(stepY, 0, 1, origin.distance, stepY.x);
//
//   if (nextStep.distance > range) return [origin];
//   return [origin].concat(ray(nextStep));
// }

// function step(rise, run, x, y, inverted) {
//          if (run === 0) return noWall;
//          var dx = run > 0 ? Math.floor(x + 1) - x : Math.ceil(x - 1) - x;
//          var dy = dx * (rise / run);
//          return {
//            x: inverted ? y + dy : x + dx,
//            y: inverted ? x + dx : y + dy,
//            length2: dx * dx + dy * dy
//          };
//        }
