trait Vehicle: Paint {
    fn park(&self); // no implementation as default
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {color}");// default implementation
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16
}
struct Car {
    info: VehicleInfo,
}

impl Vehicle for Car {
    fn park(&self) {
        println!("parking car!");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn unload(&self) {
        println!("unloading truck!");
    }
}

impl Vehicle for Truck {
    fn park(&self) {
        println!("parking truck!");
    }
}
impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house: {color}");
    }
}
fn main() {
    let car = Car {
        info: VehicleInfo { 
            make: "Honda".to_owned(), 
            model: "Civic".to_owned(), 
            year: 1995 
        }
    };
    let house = House {};
    let object = create_paintable_object(false);

    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];


    paint_red(&car);
    paint_red(&house);
    paint_red(object.as_ref());

    paint_vehicle_red(&car);
}

fn paint_red(object: &dyn Paint) {
    object.paint("red".to_owned());
}
// or
fn paint_red2(object: &impl Paint) {
    object.paint("red".to_owned());
}
// or
fn paint_red3<T>(object: &T) where T: Paint {
    object.paint("red".to_owned());
}

fn paint_vehicle_red<T>(object: &T) where T: Vehicle {
    object.paint("red".to_owned());
}

fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(House {})
    } else {
        Box::new(Car {
            info: VehicleInfo { 
                make: "Opel".to_owned(), 
                model: "Astra".to_owned(), 
                year: 2000 
            }
        })
    }
}
