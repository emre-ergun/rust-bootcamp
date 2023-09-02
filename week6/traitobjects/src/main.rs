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
    let object = create_paintable_object(true);

    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];

    paint_red(&car);
    paint_red(&house);
    paint_red(object.as_ref());


    paint_vehicle_red(&car);
}

fn paint_red(object: &dyn Paint) {
    object.paint("red".to_owned());
}

fn paint_vehicle_red<T>(object:  &T) where T: Paint + Park {
    object.paint("red".to_owned());
}

fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car{
            info: VehicleInfo { 
                make: "Honda".to_owned(), 
                model: "Civic".to_owned(), 
                year: 1995 }
        })
    } else {
        Box::new(House{})
    }
}
