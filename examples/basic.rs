/* This example is the result of combining the mouse and window code from
 * https://github.com/emoon/rust_minifb/blob/bedec9b59d9e55345f5cdad0ac040fcb20b6b9d8/examples/mouse.rs
 * and the tgl setup and rendering code from
 * https://github.com/C-Chads/tinygl/blob/e71df3beac500cfe5e974c6c1f33bd4883989fdc/SDL_Examples/menu.c
 * with the end result being a small white box that follows the cursor
 */

extern crate minifb;
extern crate tgl;

use minifb::{Key, MouseMode, Window, WindowOptions};
use tgl::zbuffer;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let frame_buf: *mut zbuffer::ZBuffer;

    unsafe {
        frame_buf = zbuffer::open(
            WIDTH as i32,
            HEIGHT as i32,
            zbuffer::MODE_RGBA,
            buffer.as_mut_ptr() as *mut std::ffi::c_void,
        );
        tgl::Init(frame_buf as *mut std::ffi::c_void);
        tgl::ClearColor(0.0, 0.0, 0.0, 0.0);
        tgl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
        tgl::Enable(tgl::DEPTH_TEST);
        tgl::MatrixMode(tgl::PROJECTION);
        tgl::LoadIdentity();
        tgl::MatrixMode(tgl::MODELVIEW);
        tgl::LoadIdentity();
    }

    init_scene();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        unsafe { tgl::Clear(tgl::COLOR_BUFFER_BIT | tgl::DEPTH_BUFFER_BIT) };
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            unsafe { tgl::Color3f(1.0, 1.0, 1.0) };
            draw_box(x / (WIDTH as f32), y / (HEIGHT as f32), 0.03, 0.03);
        }

        unsafe {
            zbuffer::copyFrameBuffer(
                frame_buf,
                buffer.as_mut_ptr() as *mut std::ffi::c_void,
                4 * WIDTH as i32,
            )
        };

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    unsafe {
        zbuffer::close(frame_buf);
        tgl::Close();
    }
}

fn init_scene() {
    let mut pos: [tgl::types::GLfloat; 4] = [5.0, 5.0, 10.0, 0.0];

    let mut white: [tgl::types::GLfloat; 4] = [1.0, 1.0, 1.0, 0.0];

    unsafe {
        tgl::Lightfv(tgl::LIGHT0, tgl::POSITION, pos.as_mut_ptr());
        tgl::Lightfv(tgl::LIGHT0, tgl::DIFFUSE, white.as_mut_ptr());
        tgl::Disable(tgl::CULL_FACE);
        tgl::Disable(tgl::BLEND);
        tgl::Disable(tgl::TEXTURE_2D);
        tgl::Disable(tgl::LIGHTING);
        tgl::Disable(tgl::DEPTH_TEST);
        tgl::DepthMask(tgl::FALSE as u8);
        tgl::ShadeModel(tgl::SMOOTH);
        //tgl::TextSize(tgl::TEXT_SIZE24x24);
        tgl::Enable(tgl::NORMALIZE);
    }
}

fn draw_box(
    x: tgl::types::GLfloat,
    y: tgl::types::GLfloat,
    xdim: tgl::types::GLfloat,
    ydim: tgl::types::GLfloat,
) {
    // 0,0 is top left, 1,1 is bottom right

    let x = x * 2.0;
    let xdim = xdim * 2.0;
    let y = y * 2.0;
    let ydim = ydim * 2.0;
    unsafe {
        tgl::Begin(tgl::TRIANGLES);
        // TRIANGLE 1,
        tgl::TexCoord2f(0.0, 0.0);
        tgl::Vertex3f(-1.0 + x, 1.0 - y - ydim, 0.5); // Bottom Left Corner

        tgl::TexCoord2f(1.0, -1.0);
        tgl::Vertex3f(-1.0 + x + xdim, 1.0 - y, 0.5); // Top Right Corner

        tgl::TexCoord2f(0.0, -1.0);
        tgl::Vertex3f(-1.0 + x, 1.0 - y, 0.5); // Top Left
                                               // TRIANGLE 2
        tgl::TexCoord2f(0.0, 0.0);
        tgl::Vertex3f(-1.0 + x, 1.0 - y - ydim, 0.5); // Bottom Left Corner

        tgl::TexCoord2f(1.0, 0.0);
        tgl::Vertex3f(-1.0 + x + xdim, 1.0 - y - ydim, 0.5);

        tgl::TexCoord2f(1.0, -1.0);
        tgl::Vertex3f(-1.0 + x + xdim, 1.0 - y, 0.5); // Top Right Corner
        tgl::End();
    }
}
