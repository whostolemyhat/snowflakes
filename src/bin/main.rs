// extern crate cairo;
extern crate snowflake;

// use std::fs::File;
// use cairo::{ Context, Format, ImageSurface };
use snowflake::{ create_hash };
use snowflake::draw::draw;

// const IMAGE_SIZE: i32 = 500;
// const SCALE: f64 = 4.0;

// fn draw_points(ctx: &Context, points: &Vec<Point>) {
//     for point in points {
//         let offset = (IMAGE_SIZE as f64 - SCALE) / 2.0;
//         let (x, y) = hex_to_cartesian(point.x().into(), point.y().into());
//         draw_hexagon(ctx, x + offset, y + offset, SCALE);
//     }

//     ctx.fill();
// }

// fn hex_to_cartesian(hex_x: f64, hex_y: f64) -> (f64, f64) {
//     (
//         SCALE * (3f64.sqrt() * hex_x + 3f64.sqrt() / 2.0 * hex_y),
//         SCALE * (3.0 / 2.0 * hex_y)
//     )
// }

// fn draw_hexagon(ctx: &Context, x: f64, y: f64, r: f64) {
//     let a = -3f64.sqrt() / 2.0 * r;
//     let b = r / 2.0;

//     ctx.move_to(a + x, y - b);
//     ctx.line_to(x, y - r);
//     ctx.line_to(x - a, y - b);
//     ctx.line_to(x - a, b + y);
//     ctx.line_to(x, r + y);
//     ctx.line_to(a + x, b + y);
//     ctx.line_to(a + x, y - b);
// }

fn main() {
    let hash = match std::env::args().nth(1) {
        Some(text) => text,
        None => create_hash(64)
    };

    match draw(&hash) {
        Ok(_) => println!("{:?}", hash),
        Err(e) => println!("Error creating image: {:?}", e)
    }

//     let flake = generate(&hash);

//     let default_output = format!("images/{}.png", hash);
//     let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
//     let ctx = Context::new(&surface);
//     draw_points(&ctx, &flake);
//     let mut file = File::create(default_output).unwrap();
//     surface.write_to_png(&mut file).unwrap();

//     println!("{:?}", hash);
}
