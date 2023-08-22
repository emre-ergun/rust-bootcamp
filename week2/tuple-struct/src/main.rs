fn main() {
    let rbg_color = (255, 106, 0);
    let cmyk_color = (0, 58, 100, 0);

    // tuple structs
    struct RGB(i32, i32, i32);
    struct  CMYK(i32, i32, i32, i32);

    let color1 = RGB(255, 106, 0);
    let color2 = CMYK(0, 58, 100, 0);

    // unit-like struct
    // whenyou have a trait that you need to implement on somethin, but
    // you don't need to store any data inside of it.
    struct MyStruct;
}
