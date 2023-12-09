use derive_traits;

struct PointWrapper(derive_traits::Point);

impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let p1  = Point {x: 3, y: 1};
    let p2  = Point {x: 3, y: 1};
    let p3  = Point {x: 5, y: 5};

    println!("{p1:#?}");
    println!("{}", p1 == p2);
    println!("{}", p1 == p3);

    let p1  = PointWrapper(derive_traits::Point {x: 3, y: 1});
    let p2  = PointWrapper(derive_traits::Point {x: 3, y: 1});

    println!("Orphan rule for point: {}", p1 == p2);
}
