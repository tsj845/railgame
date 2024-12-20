#[macro_use]
extern crate glium;

use glium::Surface;
use glium::winit::event::WindowEvent;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4]
}
implement_vertex!(Vertex, position, color);

const vertex_shader_src: &'static str = include_str!("../shaders/main.vert");

const fragment_shader_src: &'static str = include_str!("../shaders/main.frag");

fn main() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let vertex1 = Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0, 1.0] };
    let vertex2 = Vertex { position: [-0.5,  0.5, 0.0], color: [0.0, 1.0, 0.0, 1.0] };
    let vertex3 = Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0, 1.0] };
    let vertex4 = Vertex { position: [ 0.5,  0.5, 1.0], color: [1.0, 0.0, 1.0, 1.0] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &[0, 1, 2, 1, 2, 3u32]).unwrap();

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut angle: f32 = 0.0;

    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                WindowEvent::RedrawRequested => {
                    let mut frame = display.draw();
                    frame.clear_color(0.3, 0.2, 1.0, 1.0);
                    frame.draw(&vertex_buffer, &indices, &program, &uniform! { trans: [[1., 0., 0., 0.], [0., angle.cos(), -angle.sin(), angle.sin()], [0., angle.sin()/4., angle.cos()/4., -angle.cos()/4.], [0., 0., 0., 1.0f32]]}, &Default::default()).unwrap();
                    frame.finish().unwrap();
                },
                WindowEvent::KeyboardInput { event, .. } => {
                    match event.logical_key {
                        glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::ArrowRight) => angle = (angle+0.05).min(1.2),
                        glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::ArrowLeft) => angle = (angle-0.05).max(0.),
                        _ => {}
                    }
                    window.request_redraw();
                },
                _ => (),
            },
            _ => (),
        };
    });
}
