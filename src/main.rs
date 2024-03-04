use winit::{
    event::{DeviceEvent, Event, KeyEvent, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{CursorGrabMode, Fullscreen, WindowBuilder},
};

fn main() {
    let fullscreen = Some(Fullscreen::Borderless(None));
    // let fullscreen = None;

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_fullscreen(fullscreen)
        .build(&event_loop)
        .unwrap();

    window.set_cursor_grab(CursorGrabMode::Confined).unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop
        .run(|event, target| match event {
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => window.request_redraw(),

            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => target.exit(),

            Event::AboutToWait => {}
            Event::NewEvents(StartCause::WaitCancelled { .. }) => {}
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { .. },
                ..
            } => {}
            Event::DeviceEvent {
                event: DeviceEvent::Motion { .. },
                ..
            } => {}
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { .. },
                ..
            } => {}

            e => {
                println!("{e:?}");
            }
        })
        .unwrap();

    println!("Finished!");
}
