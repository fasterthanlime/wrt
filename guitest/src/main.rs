mod interop;
#[allow(unused)]
use interop::{create_dispatcher_queue_controller_for_current_thread, ro_initialize, RoInitType};

mod window_target;
#[allow(unused)]
use window_target::CompositionDesktopWindowTargetSource;

#[allow(unused)]
use windows::{foundation::numerics::Vector2, ui::composition::Compositor};

use bindings::prelude::*;
#[allow(unused)]
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> winrt::Result<()> {
    windows::application_model::core::CoreApplication::run();

    // ro_initialize(RoInitType::MultiThreaded)?;
    // let _controller = create_dispatcher_queue_controller_for_current_thread()?;

    // let event_loop = EventLoop::new();
    // let window = WindowBuilder::new().build(&event_loop).unwrap();
    // window.set_title("WinRT/Rust GUI test");

    // let compositor = Compositor::new()?;
    // let target = window.create_window_target(&compositor, false)?;

    // let root = compositor.create_container_visual()?;
    // root.set_relative_size_adjustment(Vector2 { x: 1.0, y: 1.0 })?;
    // target.set_root(&root)?;

    // event_loop.run(move |event, _, control_flow| match event {
    //     Event::WindowEvent {
    //         event: WindowEvent::CloseRequested,
    //         window_id,
    //     } if window_id == window.id() => *control_flow = ControlFlow::Exit,
    //     _ => {}
    // });

    Ok(())
}
