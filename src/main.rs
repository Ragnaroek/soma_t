mod vector;
mod geometry;

use std::io;
use std::io::Write;

use vector::Vec3f;
use geometry::{Object, Triangle};

fn out_demo() {
    print!("\x1B[?1049h"); //enter alternate screen
    print!("\x1B[H");
    print!("alternate screen mode");
    print!("\x1B\x5B9B"); //move cursor 9 lines down

    // TODO Exit alternate screen to restore terminal

    io::stdout().flush();

    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);
    println!("{:?}", result);
}

const FRAME_WIDTH : usize = 120;
const FRAME_HEIGHT: usize = 30;

const FOV: f32 = 51.52;

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

            let x_screen = 2.0 * ((x as f32) + 0.5) / ((FRAME_WIDTH as f32) - 1.0) * aspect_ratio * fov_scale;
            let y_screen = (1.0 - 2.0 * ((y as f32) + 0.5)) / (FRAME_HEIGHT as f32) * fov_scale;
            let dir = Vec3f::new(x_screen, y_screen, -1.0).normalize();

            if tri.intersect(&orig, &dir) {
                println!("ray intersection {:?},{:?}", x, y);
                frame_buffer[x][y] = true;
            }
        }
    }


    // TODO draw frame_buffer to console
}
