// https://joshleeb.com/posts/rust-wasm-snowhash/
extern crate rand;
extern crate cairo;

mod bit;
mod bitstr;

use rand::{ Rng, SeedableRng, StdRng };
use cairo::{ Context, Format, ImageSurface };
use std::fs::File;
use bitstr::BitStr;

#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    // fn new(x: i32, y: i32) -> Self {
    //     Point(x, y)
    // }

    fn origin() -> Self {
        Point(0, 0)
    }

    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
        self.1
    }

    fn on_axis(&self) -> bool {
        self.0 == 0 || self.1 == 0
    }

    fn reflection(&self) -> Vec<Self> {
        match *self {
            Point(0, 0) => vec![Point(0, 0)],
            Point(x, 0) => vec![Point(x, 0), Point(0, x), Point(-x, 0), Point(0, -x), Point(x, -x), Point(-x, x)],
            Point(x, y) => {
                let sum = x + y;
                vec![
                    Point(x, y),
                    Point(-x, -y),
                    Point(y, x),
                    Point(-y, -x),
                    Point(-x, sum),
                    Point(-y, sum),
                    Point(x, -sum),
                    Point(y, -sum),
                    Point(sum, -x),
                    Point(sum, -y),
                    Point(-sum, x),
                    Point(-sum, y)
                ]
            }
        }
    }

    fn neighbours(&self) -> Vec<Point> {
        vec![
            Point(self.0 + 1, self.1), // right
            Point(self.0 - 1, self.1), // left
            Point(self.0, self.1 + 1), // top
            Point(self.0, self.1 - 1), // bottom
            Point(self.0 - 1, self.1 + 1), // top right
            Point(self.0 + 1, self.1 - 1), // bottom left
        ]
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

fn hash_sum(hash: &str) -> usize {
    hash.as_bytes().into_iter().fold(0, |acc, byte| acc + *byte as usize)
}

fn extend(point: &Point, closed: &Vec<Point>) -> Vec<Point> {
    point.neighbours()
        .into_iter()
        .filter(|p| in_slice(p) && !closed.contains(p))
        .collect()
}

fn in_slice(point: &Point) -> bool {
    point.x() >= point.y() && point.y() >= 0
}

fn generate(hash: &str) -> Vec<Point> {
    let seed: &[_] = &[hash_sum(hash)];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let mut open = vec![Point::origin()];
    let mut closed = vec![];
    let mut filled = vec![];

    for bit in BitStr::from_str(hash) {
        let index = rng.gen_range(0, open.len());
        let point = open.remove(index);

        if point.on_axis() || bit.as_bool() {
            let mut reflection = point.reflection();
            filled.append(&mut reflection);

            let mut extension = extend(&point, &closed);
            open.append(&mut extension);
        }

        closed.push(point);
    }

    filled
}

const IMAGE_SIZE: i32 = 500;
const SCALE: f64 = 4.0;

fn draw_points(ctx: &Context, points: &Vec<Point>) {
    for point in points {
        let offset = (IMAGE_SIZE as f64 - SCALE) / 2.0;
        let (x, y) = hex_to_cartesian(point.x().into(), point.y().into());
        draw_hexagon(ctx, x + offset, y + offset, SCALE);
    }

    ctx.fill();
}

fn hex_to_cartesian(hex_x: f64, hex_y: f64) -> (f64, f64) {
    (
        SCALE * (3f64.sqrt() * hex_x + 3f64.sqrt() / 2.0 * hex_y),
        SCALE * (3.0 / 2.0 * hex_y)
    )
}

fn draw_hexagon(ctx: &Context, x: f64, y: f64, r: f64) {
    let a = -3f64.sqrt() / 2.0 * r;
    let b = r / 2.0;

    ctx.move_to(a + x, y - b);
    ctx.line_to(x, y - r);
    ctx.line_to(x - a, y - b);
    ctx.line_to(x - a, b + y);
    ctx.line_to(x, r + y);
    ctx.line_to(a + x, b + y);
    ctx.line_to(a + x, y - b);
}

fn create_hash(len: usize) -> String {
    rand::thread_rng().gen_ascii_chars().take(len).collect()
}

fn main() {
    let hash = match std::env::args().nth(1) {
        Some(text) => text,
        None => create_hash(64)
    };

    let flake = generate(&hash);

    let default_output = format!("images/{}.png", hash);
    let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
    let ctx = Context::new(&surface);
    draw_points(&ctx, &flake);
    let mut file = File::create(default_output).unwrap();
    surface.write_to_png(&mut file).unwrap();

    println!("{:?}", hash);
}
