use winit::{event_loop::EventLoop, event::WindowEvent};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new()
        .with_title("TextHunter")
        .with_theme(Some(winit::window::Theme::Dark))
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();
    //window.set_maximized(true);

    event_loop.run(move | event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    match input.virtual_keycode {
                        Some(winit::event::VirtualKeyCode::Escape) => *control_flow = winit::event_loop::ControlFlow::Exit,
                        _ => (),
                    }
                },
                WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
