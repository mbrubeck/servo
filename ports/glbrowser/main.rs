extern crate azure;
extern crate compositing;
extern crate servo;
extern crate util;
extern crate glfw;
extern crate geom;
extern crate layers;

use geom::point::TypedPoint2D;
use geom::scale_factor::ScaleFactor;
use geom::size::TypedSize2D;
use glfw::Context;
use layers::geometry::DevicePixel;
use util::geometry::ScreenPx;

struct GLBrowserView {
    glfw_window: glfw::Window,
}

impl compositing::windowing::View for GLBrowserView {
    fn make_current(&self) {
        self.glfw_window.make_current();
    }

    fn hidpi_factor(&self) -> ScaleFactor<ScreenPx, DevicePixel, f32> {
        let backing_size = self.framebuffer_size().width.get();
        let window_size = self.size().width.get();
        ScaleFactor((backing_size as f32) / window_size)
    }

    fn framebuffer_size(&self) -> TypedSize2D<DevicePixel, uint> {
        let (width, height) = self.glfw_window.get_framebuffer_size();
        TypedSize2D(width as uint, height as uint)
    }

    fn size(&self) -> TypedSize2D<ScreenPx, f32> {
        let (width, height) = self.glfw_window.get_size();
        TypedSize2D(width as f32, height as f32)
    }

    fn present(&self) {
        self.glfw_window.swap_buffers();
    }
}

fn main() {
    let opts = util::opts::Opts {
        urls: vec!("http://limpet.net/".to_string()),
        render_backend: azure::azure_hl::SkiaBackend,
        n_render_threads: 1,
        cpu_painting: false,
        tile_size: 512,
        device_pixels_per_px: None,
        time_profiler_period: None,
        memory_profiler_period: None,
        enable_experimental: false,
        layout_threads: 1,
        exit_after_load: false,
        output_file: None,
        headless: false,
        hard_fail: false,
        bubble_inline_sizes_separately: false,
        show_debug_borders: false,
        enable_text_antialiasing: true,
        trace_layout: false,
        devtools_server: false,
    };

    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    let view = GLBrowserView { glfw_window: window };

    servo::run(opts, box view);

    loop {
        glfw.poll_events();
    }
}
