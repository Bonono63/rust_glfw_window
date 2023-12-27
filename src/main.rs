extern crate glfw;

use glfw::{Context};

fn main() {
    println!("Hello, world!");

    //println!("bool!?: {}",false);

    use glfw::fail_on_errors;

    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw.create_window(600,480, "Window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    while !window.should_close()
    {
        window.swap_buffers();

        glfw.poll_events();
    }
}
