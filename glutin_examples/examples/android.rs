#![cfg(android_platform)]

use winit::event_loop::EventLoop;
use winit::platform::android::EventLoopBuilderExtAndroid;

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    glutin_examples::main(EventLoop::new())
}
