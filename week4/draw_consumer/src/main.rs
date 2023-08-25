use draw::color;

fn main() {
    draw::draw_line(32, 32);

    let color = color::RGB {
        r: 255,
        g: 23,
        b: 0155,
    };
    color::draw_line(32, 43, &color);

    let square = draw::shapes::Rectangle {
        color,
        width: 32,
        height: 32,
    };

    println!("{square:?}");
}
