use glium::Surface;

#[macro_use]
extern crate glium;

fn main() {
    use glium::glutin;

    let events_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).expect("display building failed");

    let mut target = display.draw();
    target.clear_color(0., 0., 1., 1.);
    target.finish().unwrap();

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
    })
}
