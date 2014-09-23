extern crate azure;
extern crate servo;
extern crate util;
extern crate glfw;

use glfw::Context;

fn main() {
    /*
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
    servo::run(opts);
    */

    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
    }
}
