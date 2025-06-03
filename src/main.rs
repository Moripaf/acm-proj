pub mod find_close_pairs;
pub mod points;
pub mod planes;
use std::str::FromStr;

use find_close_pairs::State;
use planes::Plane;


fn main() {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    println!("Enter the number of planes:");
    stdin.read_line(&mut buffer).unwrap();
    let num_planes: usize = buffer.trim().parse().unwrap();
    buffer.clear();
    if num_planes < 2 {
        println!("You need at least 2 planes to find near misses.");
        return;
    }
    println!("enter the length of the simulation:");
    stdin.read_line(&mut buffer).unwrap();
    let length: usize = buffer.trim().parse().unwrap();
    buffer.clear();
    println!("Enter each plane as idx x y z speed direction and separate planes by newline:");
    let mut planes: Vec<Plane> = Vec::with_capacity(num_planes);
    for _i in 0..num_planes {
        stdin.read_line(&mut buffer).unwrap();
        planes.push(Plane::from_str(&buffer).unwrap());
        buffer.clear();
    }
    let mut st = State::new(5.0, 1000, planes);
    find_close_pairs::find_all_near_misses(&mut st, length);
    let str = st.to_string();
    print!("{}", str);
    /* let res = find_close_pairs::find_closest_pair(points);
    println!(
        "the closest points where ({0},{1}) and ({2},{3}) with their distance being: {4}",
        res.p1.x, res.p1.y, res.p2.x, res.p2.y, res.distance
    ); */
}