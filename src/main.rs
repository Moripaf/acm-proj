pub mod find_close_pairs;
pub mod points;
pub mod planes;
use std::str::FromStr;

use find_close_pairs::State;
use planes::Plane;


fn main() {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    println!("enter the length of the simulation:");
    stdin.read_line(&mut buffer).unwrap();
    let length: usize = buffer.trim().parse().unwrap();
    buffer.clear();
    println!("Enter plains as idx x y z speed direction in a single line: ");
    let mut planes: Vec<Plane> = Vec::with_capacity(5);
    for i in 0..4 {
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