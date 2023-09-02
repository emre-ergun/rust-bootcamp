trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
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
    fn paint(&self, color: String) {
        println!("Painting House: {color}");
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo { 
            make: "Honda".to_owned(), 
            model: "Civic".to_owned(), 
            year: 1995 
        },
    };

    let house = House {};
    let object = create_paintable_object();

    paint_red1(&car);
    paint_red1(&house);
    paint_red1(&object);


    paint_vehicle_red(&car);
    // paint_vehicle_red(&house); //error becuase house does not implement Park
    // paint_vehicle_red(&object); //error becuase house does not implement Park
}

fn paint_red1<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}
fn paint_red2(object: & impl Paint) {
    object.paint("red".to_owned());
}

fn paint_red3<T>(object: &T) where T: Paint {
    object.paint("red".to_owned());
}

fn paint_vehicle_red<T>(object:  &T) where T: Paint + Park {
    object.paint("red".to_owned());
}

fn create_paintable_object() -> impl Paint {
    House{}
}
