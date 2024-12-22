#![allow(non_upper_case_globals)]
#[macro_use]
extern crate glium;

use std::env::args;
use std::time::Instant;

use glium::Surface;
use glium::winit::event::WindowEvent;
use railgame::logic::specs::{get_indust_spec, CarCapacity};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4]
}
implement_vertex!(Vertex, position, color);

const vertex_shader_src: &'static str = include_str!("../shaders/main.vert");

const fragment_shader_src: &'static str = include_str!("../shaders/main.frag");

#[derive(Clone, Copy, PartialEq)]
enum ScrollDirection {
    Up,
    Down,
    None
}

struct ScrollStatus {
    direction: ScrollDirection,
    start_time: Instant,
    start_angle: f32,
    scale_factor: f32,
}

impl ScrollStatus {
    fn new(scale_factor: f32) -> ScrollStatus {
        ScrollStatus{ direction: ScrollDirection::None, start_time: Instant::now(), start_angle: 0., scale_factor }
    }

    fn process_key(&mut self, key: glium::winit::event::KeyEvent) {
        match (key.logical_key, key.state) {
            (glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::ArrowUp), glium::winit::event::ElementState::Pressed) => match self.direction {
                ScrollDirection::Down => { self.start_angle = self.get_angle(); self.direction = ScrollDirection::None },
                ScrollDirection::None => { self.direction = ScrollDirection::Up; self.start_time = Instant::now() },
                _ => {}
            },
            (glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::ArrowUp), glium::winit::event::ElementState::Released) => if self.direction == ScrollDirection::Up {
                self.start_angle = self.get_angle();
                self.direction = ScrollDirection::None;
            },
            (glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::ArrowDown), glium::winit::event::ElementState::Pressed) => match self.direction {
                ScrollDirection::Up   => { self.start_angle = self.get_angle(); self.direction = ScrollDirection::None },
                ScrollDirection::None => { self.direction = ScrollDirection::Down; self.start_time = Instant::now() },
                _ => {}
            },
            (glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::ArrowDown), glium::winit::event::ElementState::Released) => if self.direction == ScrollDirection::Down {
                self.start_angle = self.get_angle();
                self.direction = ScrollDirection::None;
            },
            _ => {}
        }
    }

    fn get_angle(&self) -> f32 {
        match self.direction {
            ScrollDirection::Up   => self.start_angle + self.start_time.elapsed().as_secs_f32() * self.scale_factor,
            ScrollDirection::Down => self.start_angle - self.start_time.elapsed().as_secs_f32() * self.scale_factor,
            ScrollDirection::None => self.start_angle
        }
        .max(0.).min(1.2)
    }
}

#[allow(deprecated)]
fn main() {
    if args().len() > 1 {
        // println!("{}", serde_json::to_string(&CarCapacity::Test(0)).unwrap());
        println!("{:?}", get_indust_spec(0));
        return;
    }
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

    let mut scroll = ScrollStatus::new(1.);

    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                WindowEvent::RedrawRequested => {
                    let angle = scroll.get_angle();
                    let mut frame = display.draw();
                    frame.clear_color(0.3, 0.2, 1.0, 1.0);
                    frame.draw(&vertex_buffer, &indices, &program, &uniform! { trans: [[1., 0., 0., 0.], [0., angle.cos(), -angle.sin(), angle.sin()], [0., angle.sin()/4., angle.cos()/4., -angle.cos()/4.], [0., 0., 0., 1.0f32]]}, &Default::default()).unwrap();
                    frame.finish().unwrap();
                },
                WindowEvent::KeyboardInput { event, .. } => {
                    scroll.process_key(event);
                    window.request_redraw();
                },
                _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });
}
