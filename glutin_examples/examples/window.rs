mod support;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::platform::*;
use glutin::platform::unix::*;
use gtk::prelude::*;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    //println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());
    let area = unsafe { windowed_context.raw_handle() };
    let gl = support::load(&windowed_context.context());
    // area.connect_render(move |_, _| {
    //             gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
    //             windowed_context.swap_buffers().unwrap();
    //     gtk::Inhibit(false)
    // });


    el.run(move |event, _, control_flow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                //WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            //TODO should use Event::RedrawRequested(_) => {
            Event::MainEventsCleared => {
                area.make_current();
                gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
