#[macro_use]
extern crate glium;

use std::vec;
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

mod math;
mod simulation;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {

    const GRID_SIZE_X: usize = 150;
    const GRID_SIZE_Y: usize = 100;

    let mut display_values : [f32; GRID_SIZE_X * GRID_SIZE_Y] = [0.0 ; GRID_SIZE_X * GRID_SIZE_Y];

    let mut display_values_mutex_arc = Arc::new(Mutex::new(display_values));

    let simulation_display_values = display_values_mutex_arc.clone();

    let simulation_thread = thread::spawn(move || {
        simulation::simulation_loop(simulation_display_values, GRID_SIZE_X, GRID_SIZE_Y);
    });
    
    use glium::{glutin, Surface};
    use glium::uniforms::UniformBuffer;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let vertex1 = Vertex { position: [-1.0, 1.0] };
    let vertex2 = Vertex { position: [ -1.0,  -1.0] };
    let vertex3 = Vertex { position: [ 1.0, -1.0] };
    let vertex4 = Vertex { position: [1.0, 1.0] };

    let shape = vec![vertex1, vertex2, vertex3, vertex3, vertex4, vertex1];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut vertex_shader_src = fs::read_to_string("glsl/vertex.glsl").expect("Something went wrong reading the file");

    let mut fragment_shader_src = fs::read_to_string("glsl/fragment.glsl").expect("Something went wrong reading the file");

    fragment_shader_src.insert_str(14 as usize, String::from(
        format!("const int GRID_SIZE_X = {x};
                 const int GRID_SIZE_Y = {y};", x = GRID_SIZE_X, y = GRID_SIZE_Y)).as_str());

    let program = glium::Program::from_source(&display, vertex_shader_src.as_str(), fragment_shader_src.as_str(), None).unwrap();

    struct Data {
        time: f32,
        values: [f32]
    };

    implement_buffer_content!(Data);
    implement_uniform_block!(Data, time, values);

    let mut buffer: glium::uniforms::UniformBuffer<Data> =
              glium::uniforms::UniformBuffer::empty_unsized(&display, 4 + 4 * GRID_SIZE_X * GRID_SIZE_Y).unwrap();

    let mut closed = false;
    
    while !closed {
        {
        let mut mapping = buffer.map();
        let val = display_values_mutex_arc.lock().unwrap();
            for i in 0..GRID_SIZE_X * GRID_SIZE_Y - 1 {
                mapping.values[i] = val[i];
            }
        }
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! { ColorData: &buffer },
            &Default::default()).unwrap();
        target.finish().unwrap();
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

