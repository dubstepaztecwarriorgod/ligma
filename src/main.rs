use raylib::prelude::*;

mod ligma;


fn main() {
    ligma::leak_memory();

    let (mut r, thread) = raylib::init()
        .size(1000, 800)
        .title("Ligma image viwer")
        .build();

    let colors = ligma::parse(std::fs::read_to_string(std::env::args().nth(1).expect("Need a nfile path you rfucking stoopid head")).unwrap());

    while !r.window_should_close() {
        let mut t = r.begin_drawing(&thread);
        t.clear_background(Color::WHITE);

        for (y, row) in colors.iter().enumerate() {
            for (x, &color) in row.iter().enumerate() {
                let color: Color = color.into();
                t.draw_rectangle(x as i32, y as i32, 50, 50, color);
            }
        }
    }
}
