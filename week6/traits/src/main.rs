trait Park {
    fn park(&self);
}

trait Paint {
    fn pain(&self, color: String) {
        println!("Painting object: {color}");
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}
struct Car {
    info: VehicleInfo,
}

impl Park for Car {
    fn park(&self) {
        println!("Parking Car!");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo,
}

impl Park for Truck {
    fn park(&self) {
        println!("Parking Truck!");
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn pain(&self, color: String) {
        println!("Paintin House: {color}");
    }
}

fn main() {
    println!("Hello, world!");
}
