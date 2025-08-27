use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle};
use winit::error::OsError;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowAttributes};

use crate::private::Sealed;

pub trait GlutinEventLoop: Sealed {
    /// Create the window.
    fn create_window(&self, window_attributes: WindowAttributes) -> Result<Window, OsError>;

    /// Get a handle to the display controller of the windowing system.
    fn glutin_display_handle(&self) -> Result<DisplayHandle<'_>, HandleError>;
}

impl<T> Sealed for EventLoop<T> {}

impl<T> GlutinEventLoop for EventLoop<T> {
    fn create_window(&self, window_attributes: WindowAttributes) -> Result<Window, OsError> {
        let mut builder = winit::window::WindowBuilder::new();
        builder.window = window_attributes;
        builder.build(self)
    }

    fn glutin_display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.display_handle()
    }
}
