#![allow(non_upper_case_globals)]
#[macro_use]
extern crate glium;

use std::env::args;

use glium::Surface;
use glium::winit::event::WindowEvent;
use railgame::logic::specs::{get_indust_spec, CarCapacity};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4]
}
implement_vertex!(Vertex, position, color);

const vertex_shader_src: &'static str = include_str!("../shaders/main.vert");

const fragment_shader_src: &'static str = include_str!("../shaders/main.frag");

#[allow(deprecated)]
fn main() {
    if args().len() > 1 {
        // println!("{}", serde_json::to_string(&CarCapacity::Test(0)).unwrap());
        println!("{:?}", get_indust_spec(0));
        return;
    }
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let vertex1 = Vertex { position: [-0.5, -0.5],  color: [1.0, 0.0, 0.0, 1.0] };
    let vertex2 = Vertex { position: [ 0.0,  0.5],  color: [0.0, 1.0, 0.0, 1.0] };
    let vertex3 = Vertex { position: [ 0.5, -0.25], color: [0.0, 0.0, 1.0, 1.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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
                    let scale = 3.0 / (1.0+(2.0f32).powf(-angle));

                    let mut frame = display.draw();
                    frame.clear_color(0.0, 0.0, 1.0, 1.0);
                    frame.draw(&vertex_buffer, &indices, &program, &uniform! { trans: [[1., 0., 0., 0.], [0., 1., 0., scale], [0., 0., 1., 0.], [0., 0., 0., 1.]]}, &Default::default()).unwrap();
                    frame.finish().unwrap();
                },
                WindowEvent::KeyboardInput { event, .. } => {
                    match event.logical_key {
                        glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::ArrowRight) => angle += 0.05,
                        glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::ArrowLeft) => angle -= 0.05,
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
