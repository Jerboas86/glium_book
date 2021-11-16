use glium::Surface;

#[macro_use]
extern crate glium;

fn main() {
    use glium::glutin;

    let events_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).expect("display building failed");

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };

    let vertex2 = Vertex {
        position: [0.5, -0.25],
    };

    let vertex3 = Vertex {
        position: [0., 0.5],
    };

    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0 );
    }
    "#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;
    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    events_loop.run(move |ev, _, control_flow| {
        // Setting the event loop to 60 FPS fallback
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        if let glutin::event::Event::WindowEvent { event, .. } = ev {
            if event == glutin::event::WindowEvent::CloseRequested {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
        }

        let mut target = display.draw();
        target.clear_color(0., 0., 1., 1.);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();

        target.finish().unwrap();
    });
}
