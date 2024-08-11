struct Coordinate{
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let c = Coordinate{x: 1, y: 2, z: 3};
    println!("{} {} {}", c.x, c.y, c.z);
}
