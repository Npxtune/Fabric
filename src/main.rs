use glium::{glutin::dpi::PhysicalSize};
use std::io;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

#[macro_use]
extern crate glium;

fn main() 
{
    println!("Select a demo:");
    println!("1 => cube");
    println!("2 => moving cube");
    println!("3 => rotating cube");
    println!("4 => teapot");
    println!("5 => audio example");
    let mut demo_input = String::new();
    io::stdin()
    .read_line(&mut demo_input)
    .expect("Failed to read line!");

    let demo_number: i32 = demo_input.trim().parse().expect("Please type a number!");
    
    if demo_number == 1 {
        cube();
    }
    if demo_number == 2 {
        an_cube();
    }
    if demo_number == 3 {
        ro_cube();
    }
    if demo_number == 4 {
        teapot();
    }
    if demo_number == 5 {
        audio();
    }
    else
    {
        println!("Please select a valid demo!");
    }
}

fn cube()
 {
    println!("WIP - Fabric OpenGL Rust Rendering Engine.");
    println!("This rectangle is being rendered with 2 triangles with 3 vertexes each.\nThese triangles use different shaders in order to create the colour distinction.");
    println!("The background is black, though this can be changed as well.");

    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
    .with_resizable(false)
    .with_maximized(false)
    .with_inner_size(PhysicalSize::new(900, 900))
    .with_title("Fabric Rendering Engine");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [ -0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.5, -0.5] };
    let vertex3 = Vertex { position: [ -0.5, 0.5] };
    let shape = vec![vertex1, vertex2, vertex3];

    let _vertex1 = Vertex { position: [ 0.5, 0.5] };
    let _vertex2 = Vertex { position: [ -0.5, 0.5] };
    let _vertex3 = Vertex { position: [ 0.5, -0.5] };
    let shape2 = vec![_vertex1, _vertex2, _vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let vertex_buffer2 = glium::VertexBuffer::new(&display, &shape2).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.0, 0.0, 1.0, 1.0);
        }
    "#;

    let vertex_shader_src2 = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src2 = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let program2 = glium::Program::from_source(&display, vertex_shader_src2, fragment_shader_src2, None).unwrap();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.draw(&vertex_buffer2, &indices, &program2, &glium::uniforms::EmptyUniforms,
                     &Default::default()).unwrap();
        target.finish().unwrap();
    });
 }

 fn an_cube()
 {
    println!("WIP - Fabric OpenGL Rust Rendering Engine.");
    println!("This is an horizontally and vertically moving Square.");
    println!("The background is black, though this can be changed as well.");

    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
    .with_resizable(false)
    .with_maximized(false)
    .with_inner_size(PhysicalSize::new(900, 900))
    .with_title("Fabric Rendering Engine");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.5, -0.5] };
    let vertex3 = Vertex { position: [-0.5,  0.5] };
    let shape = vec![vertex1, vertex2, vertex3];

    let _vertex1 = Vertex { position: [ 0.5, 0.5] };
    let _vertex2 = Vertex { position: [ -0.5, 0.5] };
    let _vertex3 = Vertex { position: [ 0.5, -0.5] };
    let shape2 = vec![_vertex1, _vertex2, _vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let vertex_buffer2 = glium::VertexBuffer::new(&display, &shape2).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        uniform float t;
        uniform float u;
        void main() {
            vec2 pos = position;
            pos.x += t;
            pos.y += t;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 1.0, 0.0, 1.0);
        }
    "#;

    let vertex_shader_src2 = r#"
        #version 140
        in vec2 position;
        uniform float t;
        uniform float u;
        void main() {
            vec2 pos = position;
            pos.x += t;
            pos.y += t;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src2 = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.0, 1.0, 1.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let program2 = glium::Program::from_source(&display, vertex_shader_src2, fragment_shader_src2, None).unwrap();

    let mut t: f32 = -0.5;
    let mut is_right: bool = true;
    let square_speed: f32 = 0.0075; 
    event_loop.run(move |event, _, control_flow| {

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // we update `t`

        if is_right == true
        {
            t += square_speed;
            if t > 0.5  {
                is_right = false;
            }
        }
        else
        {
            t -= square_speed;
            if t < -0.5 {
                is_right = true;
            }
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        let uniforms = uniform! { t: t};
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.draw(&vertex_buffer2, &indices, &program2, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    });
 }

fn ro_cube()
{
    println!("Test 123");
}

 fn teapot()
 {
    println!("WIP - Fabric OpenGL Rust Rendering Engine.");
    println!("This is an 3d teapot.");
    println!("Please know that this is just an example, this rendering engine is designed for 2d only.");

    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
    .with_resizable(false)
    .with_maximized(false)
    .with_inner_size(PhysicalSize::new(900, 900))
    .with_title("Fabric Rendering Engine");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.5, -0.5] };
    let vertex3 = Vertex { position: [-0.5,  0.5] };
    let shape = vec![vertex1, vertex2, vertex3];

    let _vertex1 = Vertex { position: [ 0.5, 0.5] };
    let _vertex2 = Vertex { position: [ -0.5, 0.5] };
    let _vertex3 = Vertex { position: [ 0.5, -0.5] };
    let shape2 = vec![_vertex1, _vertex2, _vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let vertex_buffer2 = glium::VertexBuffer::new(&display, &shape2).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            vec2 pos = position;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 1.0, 0.0, 1.0);
        }
    "#;

    let vertex_shader_src2 = r#"
        #version 140
        in vec2 position;
        void main() {
            vec2 pos = position;;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src2 = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.0, 1.0, 1.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let program2 = glium::Program::from_source(&display, vertex_shader_src2, fragment_shader_src2, None).unwrap();

    event_loop.run(move |event, _, control_flow| {

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.draw(&vertex_buffer2, &indices, &program2, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    });
 }

 fn audio() 
 {
    println!("WIP - Fabric OpenGL Rust Rendering Engine.");
    println!("This rectangle is being rendered with 2 triangles with 3 vertexes each.\nThese triangles use different shaders in order to create the colour distinction.");
    println!("The background is black, though this can be changed as well.");

    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
    .with_resizable(false)
    .with_maximized(false)
    .with_inner_size(PhysicalSize::new(900, 900))
    .with_title("Fabric Rendering Engine");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [ -0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.5, -0.5] };
    let vertex3 = Vertex { position: [ -0.5, 0.5] };
    let shape = vec![vertex1, vertex2, vertex3];

    let _vertex1 = Vertex { position: [ 0.5, 0.5] };
    let _vertex2 = Vertex { position: [ -0.5, 0.5] };
    let _vertex3 = Vertex { position: [ 0.5, -0.5] };
    let shape2 = vec![_vertex1, _vertex2, _vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let vertex_buffer2 = glium::VertexBuffer::new(&display, &shape2).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.0, 0.0, 1.0, 1.0);
        }
    "#;

    let vertex_shader_src2 = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src2 = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let program2 = glium::Program::from_source(&display, vertex_shader_src2, fragment_shader_src2, None).unwrap();

    println!("\nPlaying an audio file on a new thread.");
    
    // Get a output stream handle to the default physical sound device
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("assets/sound.mp3").unwrap());
    let source = Decoder::new(file).unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    sink.append(source);
    sink.set_volume(0.4);

    println!("Press Ctrl + C to stop the playback");

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.draw(&vertex_buffer2, &indices, &program2, &glium::uniforms::EmptyUniforms,
                     &Default::default()).unwrap();
        target.finish().unwrap();
    });
 }