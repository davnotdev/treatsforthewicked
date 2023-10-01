use super::*;
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use winit::{event::Event as EventData, event_loop::EventLoop, window::Window};

pub use winit::event::{ElementState, VirtualKeyCode, WindowEvent as WindowEventData};

#[derive(Event)]
pub struct WindowEvent(pub WindowEventData<'static>);

#[derive(Event)]
pub struct WindowResize {
    pub width: usize,
    pub height: usize,
}

#[derive(SingleResource)]
pub struct RawWindowRes {
    pub display: RawDisplayHandle,
    pub window: RawWindowHandle,
    pub initial_width: usize,
    pub initial_height: usize,
}

struct WindowState {
    event_loop: EventLoop<()>,
    window: Window,
}

impl WindowState {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        insert_canvas(&window);

        Self { event_loop, window }
    }
}

pub fn window_run(mut galaxy: Galaxy, pre_updates: &[fn(&Galaxy)], systems: &[fn(&Galaxy)]) {
    let state = WindowState::new();

    let window_size = get_window_size(&state.window);

    galaxy.insert_resource(
        RawWindowRes::single_resource(),
        RawWindowRes {
            display: state.event_loop.raw_display_handle(),
            window: state.window.raw_window_handle(),
            initial_width: window_size.0,
            initial_height: window_size.1,
        },
    );

    for pre_update in pre_updates.iter() {
        pre_update(&galaxy);
    }
    galaxy.update();

    let systems = systems.to_vec();
    let mut last_window_size = window_size;
    state.event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            EventData::WindowEvent {
                event: WindowEventData::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
                galaxy.set_exit();
            }
            EventData::WindowEvent { event, .. } => {
                galaxy.insert_event(WindowEvent(event.to_static().unwrap()));
            }
            EventData::MainEventsCleared => {
                let window_size = get_window_size(&state.window);
                if last_window_size.0 != window_size.0 || last_window_size.1 != window_size.1 {
                    // Who needs it anyway!
                    // galaxy.insert_event(WindowResize {
                    //     width: window_size.0,
                    //     height: window_size.1,
                    // });
                }
                last_window_size = window_size;

                for system in systems.iter() {
                    system(&galaxy);
                }

                if galaxy.update().is_none() {
                    control_flow.set_exit();
                }
            }
            _ => (),
        }
    });
}

#[allow(unused_variables)]
fn get_window_size(window: &Window) -> (usize, usize) {
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    {
        use wasm_bindgen::*;

        let window = web_sys::window().unwrap();
        let canvas = window
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        return (
            canvas.client_width() as usize,
            canvas.client_height() as usize,
        );
    }

    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        let size = window.inner_size();
        (size.width as usize, size.height as usize)
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub fn insert_canvas(window: &Window) {
    use winit::platform::web::WindowExtWebSys;

    let canvas = window.canvas();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Set a background color for the canvas to make it easier to tell where the canvas is for debugging purposes.
    canvas.set_id("canvas");
    body.append_child(&canvas).unwrap();
}
