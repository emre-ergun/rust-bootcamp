trait Vehicle: Paint {
    fn park(&self);
    fn get_default_color() -> String {
        "black".to_owned()
    }
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


fn main() {
    println!("Hello, world!");
}

fn paint_vehicle_red<T>(object: &T) where T: Vehicle //instead of Paint + Vehicle 
{
    object.paint("red".to_owned());
}