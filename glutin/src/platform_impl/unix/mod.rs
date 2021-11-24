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
use gtk::prelude::*;
use gtk::GLArea;

use std::marker::PhantomData;
use std::os::raw;

/// Context handles available on Unix-like platforms.
pub type RawHandle = GLArea;

#[derive(Debug)]
pub struct Context(GLArea);


impl Context {
    #[inline]
    pub fn new_windowed<T>(
        wb: WindowBuilder,
        el: &EventLoopWindowTarget<T>,
        pf_reqs: &PixelFormatRequirements,
        gl_attr: &GlAttributes<&Context>,
    ) -> Result<(Window, Self), CreationError> {
        let window = wb.build(el)?;
        let gtkwin = window.gtk_window();

        // TODO config of pf_reqs and gl_attr
        let area = GLArea::new();
        let vbox = gtkwin.children().pop().unwrap().downcast::<gtk::Box>().unwrap();
        vbox.pack_start(&area, true, true, 0);
        area.grab_focus();
        gtkwin.show_all();

        gl_loader::init_gl();
        
        let context = Context(area);
        
        Ok((window, context))
    }

    #[inline]
    pub fn new_headless<T>(
        el: &EventLoopWindowTarget<T>,
        pf_reqs: &PixelFormatRequirements,
        gl_attr: &GlAttributes<&Context>,
        size: dpi::PhysicalSize<u32>,
    ) -> Result<Self, CreationError> {
        Self::new_headless_impl(el, pf_reqs, gl_attr, Some(size))
    }

    pub fn new_headless_impl<T>(
        el: &EventLoopWindowTarget<T>,
        pf_reqs: &PixelFormatRequirements,
        gl_attr: &GlAttributes<&Context>,
        size: Option<dpi::PhysicalSize<u32>>,
    ) -> Result<Self, CreationError> {
        todo!()
    }

    #[inline]
    pub unsafe fn make_current(&self) -> Result<(), ContextError> {
        self.0.make_current();
        Ok(())
    }

    #[inline]
    pub unsafe fn make_not_current(&self) -> Result<(), ContextError> {
        todo!()
    }

    #[inline]
    pub fn is_current(&self) -> bool {
        todo!()
    }

    #[inline]
    pub fn get_api(&self) -> Api {
        todo!()
    }

    #[inline]
    pub unsafe fn raw_handle(&self) -> RawHandle {
        self.0.clone()
    }

    #[inline]
    pub fn resize(&self, width: u32, height: u32) {
        // TODO
    }

    #[inline]
    pub fn get_proc_address(&self, addr: &str) -> *const core::ffi::c_void {
        gl_loader::get_proc_address(addr) as *const _
    }

    #[inline]
    pub fn swap_buffers(&self) -> Result<(), ContextError> {
        // GTK swaps the buffers after each "render" signal itself
        self.0.queue_render();
        Ok(())
    }

    #[inline]
    pub fn swap_buffers_with_damage(&self, rects: &[Rect]) -> Result<(), ContextError> {
        todo!()
    }

    #[inline]
    pub fn swap_buffers_with_damage_supported(&self) -> bool {
        todo!()
    }

    #[inline]
    pub fn get_pixel_format(&self) -> PixelFormat {
        todo!()
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

/* TODO
/// A unix-specific extension to the [`ContextBuilder`] which allows building
/// unix-specific headless contexts.
///
/// [`ContextBuilder`]: ../../struct.ContextBuilder.html
pub trait HeadlessContextExt {
    /// Builds an OsMesa context.
    ///
    /// Errors can occur if the OpenGL [`Context`] could not be created. This
    /// generally happens because the underlying platform doesn't support a
    /// requested feature.
    ///
    /// [`Context`]: struct.Context.html
    fn build_osmesa(
        self,
        size: dpi::PhysicalSize<u32>,
    ) -> Result<crate::Context<NotCurrent>, CreationError>
    where
        Self: Sized;

    /// Builds an EGL-surfaceless context.
    ///
    /// Errors can occur if the OpenGL [`Context`] could not be created. This
    /// generally happens because the underlying platform doesn't support a
    /// requested feature.
    ///
    /// [`Context`]: struct.Context.html
    fn build_surfaceless<TE>(
        self,
        el: &EventLoopWindowTarget<TE>,
    ) -> Result<crate::Context<NotCurrent>, CreationError>
    where
        Self: Sized;
}

impl<'a, T: ContextCurrentState> HeadlessContextExt for crate::ContextBuilder<'a, T> {
    #[inline]
    fn build_osmesa(
        self,
        size: dpi::PhysicalSize<u32>,
    ) -> Result<crate::Context<NotCurrent>, CreationError>
    where
        Self: Sized,
    {
        let crate::ContextBuilder { pf_reqs, gl_attr } = self;
        let gl_attr = gl_attr.map_sharing(|ctx| &ctx.context);
        Context::is_compatible(&gl_attr.sharing, ContextType::OsMesa)?;
        let gl_attr = gl_attr.clone().map_sharing(|ctx| match *ctx {
            Context::OsMesa(ref ctx) => ctx,
            _ => unreachable!(),
        });
        osmesa::OsMesaContext::new(&pf_reqs, &gl_attr, size)
            .map(|context| Context::OsMesa(context))
            .map(|context| crate::Context { context, phantom: PhantomData })
    }

    #[inline]
    fn build_surfaceless<TE>(
        self,
        el: &EventLoopWindowTarget<TE>,
    ) -> Result<crate::Context<NotCurrent>, CreationError>
    where
        Self: Sized,
    {
        let crate::ContextBuilder { pf_reqs, gl_attr } = self;
        let gl_attr = gl_attr.map_sharing(|ctx| &ctx.context);
        Context::new_headless_impl(el, &pf_reqs, &gl_attr, None)
            .map(|context| crate::Context { context, phantom: PhantomData })
    }
}

/// A unix-specific extension for the [`ContextBuilder`] which allows
/// assembling [`RawContext<T>`]s.
///
/// [`RawContext<T>`]: ../../type.RawContext.html
/// [`ContextBuilder`]: ../../struct.ContextBuilder.html
pub trait RawContextExt {
    /// Creates a raw context on the provided surface.
    ///
    /// Unsafe behaviour might happen if you:
    ///   - Provide us with invalid parameters.
    ///   - The surface/display_ptr is destroyed before the context
    #[cfg(feature = "wayland")]
    unsafe fn build_raw_wayland_context(
        self,
        display_ptr: *const wayland::wl_display,
        surface: *mut raw::c_void,
        width: u32,
        height: u32,
    ) -> Result<crate::RawContext<NotCurrent>, CreationError>
    where
        Self: Sized;

    /// Creates a raw context on the provided window.
    ///
    /// Unsafe behaviour might happen if you:
    ///   - Provide us with invalid parameters.
    ///   - The xwin is destroyed before the context
    #[cfg(feature = "x11")]
    unsafe fn build_raw_x11_context(
        self,
        xconn: Arc<XConnection>,
        xwin: raw::c_ulong,
    ) -> Result<crate::RawContext<NotCurrent>, CreationError>
    where
        Self: Sized;
}

impl<'a, T: ContextCurrentState> RawContextExt for crate::ContextBuilder<'a, T> {
    #[inline]
    #[cfg(feature = "wayland")]
    unsafe fn build_raw_wayland_context(
        self,
        display_ptr: *const wayland::wl_display,
        surface: *mut raw::c_void,
        width: u32,
        height: u32,
    ) -> Result<crate::RawContext<NotCurrent>, CreationError>
    where
        Self: Sized,
    {
        let crate::ContextBuilder { pf_reqs, gl_attr } = self;
        let gl_attr = gl_attr.map_sharing(|ctx| &ctx.context);
        Context::is_compatible(&gl_attr.sharing, ContextType::Wayland)?;
        let gl_attr = gl_attr.clone().map_sharing(|ctx| match *ctx {
            Context::Wayland(ref ctx) => ctx,
            _ => unreachable!(),
        });
        wayland::Context::new_raw_context(display_ptr, surface, width, height, &pf_reqs, &gl_attr)
            .map(|context| Context::Wayland(context))
            .map(|context| crate::Context { context, phantom: PhantomData })
            .map(|context| crate::RawContext { context, window: () })
    }

    #[inline]
    #[cfg(feature = "x11")]
    unsafe fn build_raw_x11_context(
        self,
        xconn: Arc<XConnection>,
        xwin: raw::c_ulong,
    ) -> Result<crate::RawContext<NotCurrent>, CreationError>
    where
        Self: Sized,
    {
        let crate::ContextBuilder { pf_reqs, gl_attr } = self;
        let gl_attr = gl_attr.map_sharing(|ctx| &ctx.context);
        Context::is_compatible(&gl_attr.sharing, ContextType::X11)?;
        let gl_attr = gl_attr.clone().map_sharing(|ctx| match *ctx {
            Context::X11(ref ctx) => ctx,
            _ => unreachable!(),
        });
        x11::Context::new_raw_context(xconn, xwin, &pf_reqs, &gl_attr)
            .map(|context| Context::X11(context))
            .map(|context| crate::Context { context, phantom: PhantomData })
            .map(|context| crate::RawContext { context, window: () })
    }
}
*/
