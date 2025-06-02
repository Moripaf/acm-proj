use closest_pair_points::find_closest_pair;
use closest_pair_points::points::Point;
fn main() {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    println!("enter the number of points you want:");
    stdin.read_line(&mut buffer).unwrap();
    let number: usize = buffer.trim().parse().unwrap();
    buffer.clear();
    println!("Enter points as x y in a single line: ");
    stdin.read_line(&mut buffer).unwrap();
    let mut points: Vec<Point> = Vec::with_capacity(number);
    let mut is_x = true;
    let mut coords_x = 0.0;
    buffer.trim().split(" ").for_each(|x| {
        println!("{}", x);
        if is_x {
            coords_x = x.parse::<f32>().unwrap_or_else(|e| {
                println!("error at x");
                0.0
            });
        } else {
            points.push(Point::new(
                coords_x,
                x.parse::<f32>().unwrap_or_else(|e| {
                    println!("error at y");
                    0.0
                }),
            ));
        };
        is_x = !is_x;
    });
    let res = find_closest_pair(points);
    println!(
        "the closest points where ({0},{1}) and ({2},{3}) with their distance being: {4}",
        res.p1.x, res.p1.y, res.p2.x, res.p2.y, res.distance
    );
}