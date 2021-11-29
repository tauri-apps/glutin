#[cfg(any(target_os = "linux", target_os = "windows"))]
mod support;

fn main() {
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    unimplemented!();
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    this_example::main();
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
mod this_example {
    use super::support;
    use glutin::event::{Event, WindowEvent};
    use glutin::event_loop::{ControlFlow, EventLoop};
    use glutin::platform::ContextTraitExt;
    use glutin::window::WindowBuilder;
    use glutin::ContextBuilder;
    use std::io::Write;
    use takeable_option::Takeable;

    pub fn main() {
        print!("Do you want transparency? (true/false) (default: true): ");
        std::io::stdout().flush().unwrap();

        let mut transparency = String::new();
        std::io::stdin().read_line(&mut transparency).unwrap();
        let transparency = transparency.trim().parse().unwrap_or_else(|_| {
            println!("Unknown input, assumming true.");
            true
        });

        let (raw_context, el, _win) = {
            let el = EventLoop::new();
            let mut wb = WindowBuilder::new().with_title("A fantastic window!");

            if transparency {
                wb = wb.with_decorations(false).with_transparent(true);
            }

            #[cfg(target_os = "linux")]
            unsafe {
                use glutin::platform::unix::RawContextExt;

                let win = wb.build(&el).unwrap();
                let raw_context =
                    ContextBuilder::new().build_raw_context(&win).unwrap();

                (raw_context, el, win)
            }

            #[cfg(target_os = "windows")]
            unsafe {
                let win = wb.build(&el).unwrap();
                use glutin::platform::windows::{RawContextExt, WindowExtWindows};

                let hwnd = win.hwnd();
                let raw_context = ContextBuilder::new().build_raw_context(hwnd).unwrap();

                (raw_context, el, win)
            }
        };

        let raw_context = unsafe { raw_context.make_current().unwrap() };

        //println!("Pixel format of the window's GL context: {:?}", raw_context.get_pixel_format());

        let gl = support::load(&*raw_context);

        #[cfg(target_os = "linux")]
        {
            let glarea = unsafe { raw_context.raw_handle() };

            use gtk::prelude::*;
            glarea.connect_render(move |_, _| {
                gl.draw_frame(if transparency { [0.0; 4] } else { [1.0, 0.5, 0.7, 1.0] });
                gtk::Inhibit(false)
            });
        }

        let mut raw_context = Takeable::new(raw_context);


        el.run(move |event, _, control_flow| {
            println!("{:?}", event);
            *control_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => {
                    Takeable::take(&mut raw_context); // Make sure it drops first
                    return;
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(physical_size) => raw_context.resize(physical_size),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                #[cfg(not(target_os = "linux"))]
                Event::RedrawRequested(_) => {
                    gl.draw_frame(if transparency { [0.0; 4] } else { [1.0, 0.5, 0.7, 1.0] });
                    raw_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
