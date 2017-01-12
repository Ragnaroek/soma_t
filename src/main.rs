mod vector;
mod geometry;

use std::io;
use std::io::Write;

use vector::Vec3f;

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

    //a very primitive frame-buffer for now: only visible or not
    let frame_buffer = [false; FRAME_WIDTH*FRAME_HEIGHT];

    //fov scale
    let fov_scale =  FOV.to_radians().tan();
    let aspect_ratio = (FRAME_WIDTH / FRAME_HEIGHT) as f32;

    for x in 0..FRAME_WIDTH {
        for y in 0..FRAME_HEIGHT {

            let x_screen = ((x as f32) + 0.5) / (FRAME_WIDTH as f32) * aspect_ratio * fov_scale;
            let y_screen = ((y as f32) + 0.5) / (FRAME_WIDTH as f32) * fov_scale;

            // - check if ray intersects triangle and toggle frame_buffer pixel
        }
    }

    // TODO draw frame_buffer to console
}
