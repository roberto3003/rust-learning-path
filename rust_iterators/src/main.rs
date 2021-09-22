struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let points = vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }];
    let _first_point: Point = points.into_iter().next().unwrap();

    let points = vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }];
    let mut iter = points.iter();
    let _first_point: &Point = iter.next().unwrap();

    let mut points = vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }];
    let mut iter = points.iter.mut();
    let first_point: &mut Point = iter.next().unwrap();

    first_point.x = 3.0;
    first_point.y = 4.0;
    println!("{:?}", points);
}
