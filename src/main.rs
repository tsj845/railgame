#![allow(non_upper_case_globals)]
#[macro_use]
extern crate glium;

use std::env::args;
use std::time::Instant;

use glium::Surface;
use glium::winit::event::WindowEvent;
use railgame::logic::specs::{ensure_init, get_indust_spec};
use railgame::logic::world::World;
use railgame::logic::world::MIC_SCALE;

#[derive(Debug, Copy, Clone)]
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
        ensure_init();
        // println!("{}", serde_json::to_string(&CarCapacity::Test(0)).unwrap());
        println!("{:?}", get_indust_spec(0));
        // println!("{:#?}", World::from_jsonstr(include_str!("../assets/sculpted/test.json")));
        return;
    }

    let world = World::from_jsonstr(include_str!("../assets/sculpted/test.json"));

    let mut vertices = Vec::new();
    let mut indices  = Vec::new();

    let width  = world.width as usize;
    let height = world.height as usize;

    for x in 0..width {
        for y in 0..height {
            for mic_x in 0..MIC_SCALE {
                for mic_y in 0..MIC_SCALE {
                    let (left, left_mic) = if mic_x == 0 { if x > 0 { (x - 1, MIC_SCALE-1) } else { (x, mic_x) } } else { (x, mic_x-1) };
                    let (right, right_mic) = if mic_x == MIC_SCALE-1 { if x < width-1 { (x + 1, 0) } else { (x, mic_x) } } else { (x, mic_x+1) };
                    let (top, top_mic) = if mic_y == 0 { if y > 0 { (y - 1, MIC_SCALE-1) } else { (y, mic_y) } } else { (y, mic_y-1) };
                    let (bottom, bottom_mic) = if mic_y == MIC_SCALE-1 { if y < height-1 { (y + 1, 0) } else { (y, mic_y) } } else { (y, mic_y+1) };

                    let base_x = (x * MIC_SCALE) as f32 + mic_x as f32;
                    let base_y = (y * MIC_SCALE) as f32 + mic_y as f32;

                    // find the positions of the five vertices
                    let pos_tl = [base_x-2.5, base_y,
                                  ( world.grid[x+y*width].subs[mic_x+mic_y*MIC_SCALE].elevation as f32 +
                                    world.grid[left+y*width].subs[left_mic+mic_y*MIC_SCALE].elevation as f32 +
                                    world.grid[left+top*width].subs[left_mic+top_mic*MIC_SCALE].elevation as f32 +
                                    world.grid[x+top*width].subs[mic_x+top_mic*MIC_SCALE].elevation as f32
                                  ) / 1024.];
                    let pos_tr = [base_x-1.5, base_y,
                                  ( world.grid[x+y*width].subs[mic_x+mic_y*MIC_SCALE].elevation as f32 +
                                    world.grid[right+y*width].subs[right_mic+mic_y*MIC_SCALE].elevation as f32+
                                    world.grid[right+top*width].subs[right_mic+top_mic*MIC_SCALE].elevation as f32+
                                    world.grid[x+top*width].subs[mic_x+top_mic*MIC_SCALE].elevation as f32
                                  ) / 1024.];
                    let pos_bl = [base_x-2.5, base_y+1.,
                                  ( world.grid[x+y*width].subs[mic_x+mic_y*MIC_SCALE].elevation as f32 +
                                    world.grid[left+y*width].subs[left_mic+mic_y*MIC_SCALE].elevation as f32 +
                                    world.grid[left+bottom*width].subs[left_mic+bottom_mic*MIC_SCALE].elevation as f32+
                                    world.grid[x+bottom*width].subs[mic_x+bottom_mic*MIC_SCALE].elevation as f32
                                  ) / 1024.];
                    let pos_br = [base_x-1.5, base_y+1.,
                                  ( world.grid[x+y*width].subs[mic_x+mic_y*MIC_SCALE].elevation as f32 +
                                    world.grid[right+y*width].subs[right_mic+mic_y*MIC_SCALE].elevation as f32 +
                                    world.grid[right+bottom*width].subs[right_mic+bottom_mic*MIC_SCALE].elevation as f32 +
                                    world.grid[x+bottom*width].subs[mic_x+bottom_mic*MIC_SCALE].elevation as f32
                                  ) / 1024.];
                    let pos_c  = [base_x+0.5, base_y+0.5,
                                  world.grid[x+y*width].subs[mic_x+mic_y*MIC_SCALE].elevation as f32 /256.];

                    let base_index = vertices.len() as u16;
                    vertices.push(Vertex { position: pos_tl, color: [(mic_x as f32 / MIC_SCALE as f32), 0., (mic_y as f32 / MIC_SCALE as f32), 1.] });
                    vertices.push(Vertex { position: pos_tr, color: [(mic_x as f32 / MIC_SCALE as f32), 0., (mic_y as f32 / MIC_SCALE as f32), 1.] });
                    vertices.push(Vertex { position: pos_c , color: [(mic_x as f32 / MIC_SCALE as f32), 0., (mic_y as f32 / MIC_SCALE as f32), 1.] });
                    vertices.push(Vertex { position: pos_bl, color: [(mic_x as f32 / MIC_SCALE as f32), 0., (mic_y as f32 / MIC_SCALE as f32), 1.] });
                    vertices.push(Vertex { position: pos_br, color: [(mic_x as f32 / MIC_SCALE as f32), 0., (mic_y as f32 / MIC_SCALE as f32), 1.] });

                    // tl - c - tr
                    indices.push(base_index);
                    indices.push(base_index+2);
                    indices.push(base_index+1);

                    // tl - c - bl
                    indices.push(base_index);
                    indices.push(base_index+2);
                    indices.push(base_index+3);

                    // tr - c - br
                    indices.push(base_index+1);
                    indices.push(base_index+2);
                    indices.push(base_index+4);

                    // bl - c - br
                    indices.push(base_index+3);
                    indices.push(base_index+2);
                    indices.push(base_index+4);
                }
            }
        }
    }

    println!("{:?}", vertices);

    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let index_buffer  = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

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
                    frame.draw(&vertex_buffer, &index_buffer, &program, &uniform! { trans: [[1., 0., 0., 0.], [0., angle.cos(), -angle.sin(), angle.sin()], [0., angle.sin()/4., angle.cos()/4., -angle.cos()/4.], [0., 0., 0., 1.0f32]]}, &Default::default()).unwrap();
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
