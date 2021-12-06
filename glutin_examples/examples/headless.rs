mod support;

use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::{
    Context, ContextBuilder, ContextCurrentState, CreationError, GlProfile, GlRequest, NotCurrent,
};
use std::path::Path;
use support::gl;


fn build_context_headless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &EventLoop<()>,
) -> Result<Context<NotCurrent>, CreationError> {
    let size_one = PhysicalSize::new(1, 1);
    cb.build_headless(&el, size_one)
}

fn build_context<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<(Context<NotCurrent>, EventLoop<()>), CreationError> {
    let el = EventLoop::new();
    build_context_headless(cb.clone(), &el).map(|ctx| (ctx, el))
}

fn main() {
    let cb = ContextBuilder::new().with_gl_profile(GlProfile::Core).with_gl(GlRequest::Latest);
    let size = PhysicalSize::new(768., 480.);

    let (headless_context, _el) = build_context(cb).unwrap();

    let headless_context = unsafe { headless_context.make_current().unwrap() };

    let gl = support::load(&headless_context);

    let mut fb = 0;
    let mut render_buf = 0;
    unsafe {
        // Using the fb backing a pbuffer is very much a bad idea. Fails on
        // many platforms, and is deprecated. Better just make your own fb.
        //
        // Surfaceless doesn't come with a surface, as the name implies, so
        // you must make your own fb.
        //
        // Making an fb is not neccesary with osmesa, however, can't be bothered
        // to have a different code path.
        gl.gl.GenRenderbuffers(1, &mut render_buf);
        gl.gl.BindRenderbuffer(gl::RENDERBUFFER, render_buf);
        gl.gl.RenderbufferStorage(gl::RENDERBUFFER, gl::RGB8, size.width as _, size.height as _);
        gl.gl.GenFramebuffers(1, &mut fb);
        gl.gl.BindFramebuffer(gl::FRAMEBUFFER, fb);
        gl.gl.FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::RENDERBUFFER,
            render_buf,
        );

        gl.gl.Viewport(0, 0, size.width as _, size.height as _);
    }

    gl.draw_frame([1.0, 0.5, 0.7, 1.0]);

    let mut pixels: Vec<gl::types::GLubyte> = vec![];
    pixels.resize(3 * size.width as usize * size.height as usize, 0);
    unsafe {
        gl.gl.ReadPixels(
            0,
            0,
            size.width as _,
            size.height as _,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            pixels.as_mut_ptr() as *mut _,
        );
    }

    let mut pixels_flipped: Vec<gl::types::GLubyte> = vec![];
    for v in (0..size.height as _).rev() {
        let s = 3 * v as usize * size.width as usize;
        let o = 3 * size.width as usize;
        pixels_flipped.extend_from_slice(&pixels[s..(s + o)]);
    }

    image::save_buffer(
        &Path::new("headless.png"),
        &pixels_flipped,
        size.width as u32,
        size.height as u32,
        image::RGB(8),
    )
    .unwrap();

    unsafe {
        gl.gl.DeleteFramebuffers(1, &fb);
        gl.gl.DeleteRenderbuffers(1, &render_buf);
    }
}
