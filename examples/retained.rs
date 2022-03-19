extern crate softbuffer;
extern crate tgl;
extern crate winit;

use softbuffer::GraphicsContext;
use tgl::zbuffer;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(window) }.unwrap();
    let (width, height) = {
        let size = graphics_context.window().inner_size();
        (size.width, size.height)
    };

    let mut buffer = vec![0; (width * height) as usize];
    let frame_buf: *mut zbuffer::ZBuffer;

    unsafe {
        frame_buf = zbuffer::open(
            width as i32,
            height as i32,
            zbuffer::MODE_RGBA,
            buffer.as_mut_ptr() as *mut std::ffi::c_void,
        );
        tgl::Init(frame_buf as *mut std::ffi::c_void);
        tgl::ClearColor(0.0, 0.0, 0.0, 0.0);
        tgl::Viewport(0, 0, width as i32, height as i32);
        tgl::Enable(tgl::DEPTH_TEST);
        tgl::MatrixMode(tgl::PROJECTION);
        tgl::LoadIdentity();
        tgl::MatrixMode(tgl::MODELVIEW);
        tgl::LoadIdentity();
    }

    init_scene();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        let (width, height) = {
            let size = graphics_context.window().inner_size();
            (size.width, size.height)
        };

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == graphics_context.window().id() => {
                unsafe {
                    zbuffer::close(frame_buf);
                    tgl::Close();
                }
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                        ..
                    },
                ..
            } => {
                unsafe { tgl::Clear(tgl::COLOR_BUFFER_BIT | tgl::DEPTH_BUFFER_BIT) };
                draw_box(
                    (position.x / (width as f64)) as tgl::types::GLfloat,
                    (position.y / (height as f64)) as tgl::types::GLfloat,
                    0.03,
                    0.03,
                );
                graphics_context.window().request_redraw();
            }
            Event::RedrawRequested(window_id) if window_id == graphics_context.window().id() => {
                unsafe {
                    zbuffer::copyFrameBuffer(
                        frame_buf,
                        buffer.as_mut_ptr() as *mut std::ffi::c_void,
                        4 * width as i32,
                    )
                };
                graphics_context.set_buffer(&buffer, width as u16, height as u16);
            }
            _ => (),
        }
    });
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

    let verts: [tgl::types::GLfloat; 18] = [
        -1.0 + x,
        1.0 - y - ydim,
        0.5,
        -1.0 + x + xdim,
        1.0 - y,
        0.5,
        -1.0 + x,
        1.0 - y,
        0.5,
        -1.0 + x,
        1.0 - y - ydim,
        0.5,
        -1.0 + x + xdim,
        1.0 - y - ydim,
        0.5,
        -1.0 + x + xdim,
        1.0 - y,
        0.5,
    ];

    let colors: [tgl::types::GLfloat; 18] = [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ];

    let texcoords: [tgl::types::GLfloat; 12] = [
        0.0, 0.0, 1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 1.0, -1.0,
    ];

    let mut buf: [tgl::types::GLuint; 3] = [0, 0, 0];
    unsafe {
        tgl::GenBuffers(3, buf.as_mut_ptr());
        tgl::BindBuffer(tgl::ARRAY_BUFFER, buf[0]);
        tgl::BufferData(
            tgl::ARRAY_BUFFER,
            std::mem::size_of::<[tgl::types::GLfloat; 18]>() as isize,
            verts.as_ptr() as _,
            tgl::STATIC_DRAW,
        );
        tgl::BindBufferAsArray(tgl::VERTEX_BUFFER, buf[0], tgl::FLOAT, 3, 0);
        tgl::BindBuffer(tgl::ARRAY_BUFFER, buf[1]);
        tgl::BufferData(
            tgl::ARRAY_BUFFER,
            std::mem::size_of::<[tgl::types::GLfloat; 18]>() as isize,
            colors.as_ptr() as _,
            tgl::STATIC_DRAW,
        );
        tgl::BindBufferAsArray(tgl::COLOR_BUFFER, buf[1], tgl::FLOAT, 3, 0);
        tgl::BindBuffer(tgl::ARRAY_BUFFER, buf[2]);
        tgl::BufferData(
            tgl::ARRAY_BUFFER,
            std::mem::size_of::<[tgl::types::GLfloat; 12]>() as isize,
            texcoords.as_ptr() as _,
            tgl::STATIC_DRAW,
        );
        tgl::BindBufferAsArray(tgl::TEXTURE_COORD_BUFFER, buf[2], tgl::FLOAT, 2, 0);
        tgl::DrawArrays(tgl::TRIANGLES, 0, 6);
        tgl::DeleteBuffers(3, buf.as_mut_ptr());
    }
}
