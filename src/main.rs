mod vector;
mod geometry;

use std::io;
use std::fs::OpenOptions;
use std::io::{Write, Error};

use vector::Vec3f;
use geometry::{Object, Triangle};

const FRAME_WIDTH : usize = 640;//220;//120;
const FRAME_HEIGHT: usize = 480;//53;//30;
const FOV: f32 = 51.52;

const FILE_OUT : bool = true; // TODO program argument

fn draw_buffer(buffer: &[[bool; FRAME_HEIGHT]; FRAME_WIDTH]) {

    print!("\x1B[?1049h"); //enter alternate screen mode
    print!("\x1B[H");

    for y in 0..FRAME_HEIGHT {
        for x in 0..FRAME_WIDTH {
            if buffer[x][y] {
                print!("\u{2588}");
            }
            print!("\x1B\x5B1C"); // move right one column (1C)
        }
        print!("\x1B\x5B1B"); //move down one row (1B)
        print!("\x1B\x5B{}D", FRAME_WIDTH);
    }

    // TODO Exit alternate screen to restore terminal

    io::stdout().flush().ok().expect("Could not flush stdout");;
    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);
    println!("{:?}", result);
}

fn write_to_ppm(buffer: &[[bool; FRAME_HEIGHT]; FRAME_WIDTH]) -> Result<(), Error> {
    let mut o = OpenOptions::new();
    let fso = o.write(true).create(true);
    let mut export = try!(fso.open("./render.ppm"));
    try!(export.write_all(b"P6\n"));
    try!(export.write_all(format!("{} ", FRAME_WIDTH).as_bytes()));
    try!(export.write_all(format!("{} ", FRAME_HEIGHT).as_bytes()));
    try!(export.write_all(b"255\n"));

    for y in 0..FRAME_HEIGHT {
        for x in 0..FRAME_WIDTH {
            if buffer[x][y] {
                try!(export.write_all(&[255, 255, 255]));
            } else {
                try!(export.write_all(&[0, 0, 0]));
            }
        }
    }

    try!(export.flush());
    return Ok(());
}

fn main() {

    // dummy geometry
    let v0 = Vec3f::new_i32(-1, -1, -5);
    let v1 = Vec3f::new_i32( 1, -1, -5);
    let v2 = Vec3f::new_i32( 0, 1, -5);
    let tri = Triangle::new(v0, v1, v2);

    //a very primitive frame-buffer for now: only visible or not
    let mut frame_buffer = [[false; FRAME_HEIGHT]; FRAME_WIDTH];

    //fov scale
    let fov_scale =  FOV.to_radians().tan();
    let aspect_ratio = (FRAME_WIDTH / FRAME_HEIGHT) as f32;

    let orig = Vec3f::new_i32(0, 0, 0);

    for x in 0..FRAME_WIDTH {
        for y in 0..FRAME_HEIGHT {

            let x_screen = (2.0 * ((x as f32) + 0.5) / (FRAME_WIDTH as f32) - 1.0) * aspect_ratio * fov_scale;
            let y_screen = ((1.0 - 2.0 * ((y as f32) + 0.5)) / (FRAME_HEIGHT as f32)) * fov_scale;
            let dir = Vec3f::new(x_screen, y_screen, -1.0).normalize();

            if tri.intersect(&orig, &dir) {
                frame_buffer[x][y] = true;
            }
        }
    }

    if FILE_OUT {
        println!("Writing to ppm file");
        let result = write_to_ppm(&frame_buffer); // TODO check error
        if result.is_ok() {
            println!("Done");
        } else {
            println!("Error {:?}", result);
        }
    } else {
        draw_buffer(&frame_buffer);
    }
}
