#![cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
))]

use crate::{
    Api, ContextCurrentState, ContextError, CreationError, GlAttributes, NotCurrent, PixelFormat,
    PixelFormatRequirements, Rect,
};

use tao::dpi;
use tao::event_loop::EventLoopWindowTarget;
use tao::window::{Window, WindowBuilder};
use tao::platform::unix::*;
use gdk::GLContext;
use gtk::prelude::*;
use gtk::GLArea;
use gtk::builders::GLAreaBuilder;

use std::marker::PhantomData;

/// Context handles available on Unix-like platforms.
pub type RawHandle = GLArea;

#[derive(Debug)]
pub struct Context {
    inner: GLArea,
    pf: PixelFormat,
}


impl Context {
    #[inline]
    pub fn new_windowed<T>(
        wb: WindowBuilder,
        el: &EventLoopWindowTarget<T>,
        pf_reqs: &PixelFormatRequirements,
        gl_attr: &GlAttributes<&Context>,
    ) -> Result<(Window, Self), CreationError> {
        let pixel_format = PixelFormat {
            hardware_accelerated: pf_reqs.hardware_accelerated.unwrap_or(true),
            color_bits: pf_reqs.color_bits.unwrap_or(24),
            alpha_bits: pf_reqs.alpha_bits.unwrap_or(8),
            depth_bits: pf_reqs.depth_bits.unwrap_or(24),
            double_buffer: pf_reqs.double_buffer.unwrap_or(true),
            multisampling: pf_reqs.multisampling,
            srgb: pf_reqs.srgb,
            stencil_bits: pf_reqs.stencil_bits.unwrap_or(8),
            stereoscopy: pf_reqs.stereoscopy,
        };
        let window = wb.build(el)?;
        let gtkwin = window.gtk_window();

        // TODO config of pf_reqs and gl_attr
        let area = GLAreaBuilder::new().has_alpha(true).build();
        let vbox = gtkwin.children().pop().unwrap().downcast::<gtk::Box>().unwrap();
        vbox.pack_start(&area, true, true, 0);
        area.grab_focus();
        gtkwin.show_all();

        gl_loader::init_gl();
        
        let context = Context {
            inner: area,
            pf: pixel_format,
        };
        
        Ok((window, context))
    }

    #[inline]
    pub fn new_headless<T>(
        _el: &EventLoopWindowTarget<T>,
        pf_reqs: &PixelFormatRequirements,
        gl_attr: &GlAttributes<&Context>,
        size: dpi::PhysicalSize<u32>,
    ) -> Result<Self, CreationError> {
        let pixel_format = PixelFormat {
            hardware_accelerated: pf_reqs.hardware_accelerated.unwrap_or(true),
            color_bits: pf_reqs.color_bits.unwrap_or(24),
            alpha_bits: pf_reqs.alpha_bits.unwrap_or(8),
            depth_bits: pf_reqs.depth_bits.unwrap_or(24),
            double_buffer: pf_reqs.double_buffer.unwrap_or(true),
            multisampling: pf_reqs.multisampling,
            srgb: pf_reqs.srgb,
            stencil_bits: pf_reqs.stencil_bits.unwrap_or(8),
            stereoscopy: pf_reqs.stereoscopy,
        };

        // TODO config of pf_reqs and gl_attr
        let area = GLAreaBuilder::new().has_alpha(true).build();
        area.grab_focus();
        gl_loader::init_gl();

        Ok(Context {
            inner: area,
            pf: pixel_format,
        })
    }

    #[inline]
    pub unsafe fn make_current(&self) -> Result<(), ContextError> {
        self.inner.make_current();
        Ok(())
    }

    #[inline]
    pub unsafe fn make_not_current(&self) -> Result<(), ContextError> {
        GLContext::clear_current();
        Ok(())
    }

    #[inline]
    pub fn is_current(&self) -> bool {
        self.inner.context() == GLContext::current()
    }

    #[inline]
    pub fn get_api(&self) -> Api {
        // TODO detect es
        Api::OpenGl
    }

    #[inline]
    pub unsafe fn raw_handle(&self) -> RawHandle {
        self.inner.clone()
    }

    #[inline]
    pub fn resize(&self, _width: u32, _height: u32) {
        // Ignored because widget will be resized automatically
    }

    #[inline]
    pub fn get_proc_address(&self, addr: &str) -> *const core::ffi::c_void {
        gl_loader::get_proc_address(addr) as *const _
    }

    #[inline]
    pub fn swap_buffers(&self) -> Result<(), ContextError> {
        // GTK swaps the buffers after each "render" signal itself
        self.inner.queue_render();
        Ok(())
    }

    #[inline]
    pub fn swap_buffers_with_damage(&self, _rects: &[Rect]) -> Result<(), ContextError> {
        Err(ContextError::FunctionUnavailable)
    }

    #[inline]
    pub fn swap_buffers_with_damage_supported(&self) -> bool {
        false
    }

    #[inline]
    pub fn get_pixel_format(&self) -> PixelFormat {
        self.pf.clone()
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

/// A unix-specific extension for the [`ContextBuilder`] which allows
/// assembling [`RawContext<T>`]s.
///
/// [`RawContext<T>`]: ../../type.RawContext.html
/// [`ContextBuilder`]: ../../struct.ContextBuilder.html
pub trait RawContextExt {
    /// Creates a raw context on the provided window/container.
    ///
    /// Unsafe behaviour might happen if you:
    ///   - Provide us with invalid parameters.
    ///   - The window/container is destroyed before the context
    unsafe fn build_raw_context(
        self,
        win: &Window,
    ) -> Result<crate::RawContext<NotCurrent>, CreationError>
    where
        Self: Sized;
}

impl<'a, T: ContextCurrentState> RawContextExt for crate::ContextBuilder<'a, T> {
    #[inline]
    unsafe fn build_raw_context(
        self,
        win: &Window,
    ) -> Result<crate::RawContext<NotCurrent>, CreationError>
    where
        Self: Sized,
    {
        let pixel_format = PixelFormat {
            hardware_accelerated: true,
            color_bits: 24,
            alpha_bits: 8,
            depth_bits: 24,
            double_buffer: true,
            multisampling: None,
            srgb: true,
            stencil_bits: 8,
            stereoscopy: false,
        };

        let gtkwin = win.gtk_window();
        let area = GLAreaBuilder::new().has_alpha(true).build();
        let vbox = gtkwin.children().pop().unwrap().downcast::<gtk::Box>().unwrap();
        vbox.pack_start(&area, true, true, 0);
        gtkwin.show_all();

        gl_loader::init_gl();

        let context = crate::Context { context: Context { inner: area, pf: pixel_format }, phantom: PhantomData };

        Ok(crate::RawContext { context, window: () })
    }
}
