mod support;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("A transparent window!")
        .with_decorations(false)
        .with_transparent(true);

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    // TODO println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    let gl = support::load(&windowed_context.context());

    #[cfg(target_os = "linux")]
    {
        use glutin::platform::*;
        use gtk::prelude::*;
        let area = unsafe { windowed_context.raw_handle() };
        area.connect_render(move |_, _| {
            gl.draw_frame([0.0; 4]);
            gtk::Inhibit(false)
        });
    }

    el.run(move |event, _, control_flow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            #[cfg(not(target_os = "linux"))]
            Event::RedrawRequested(_) => {
                gl.draw_frame([0.0; 4]);
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
