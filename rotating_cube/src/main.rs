extern crate rotating_cube;

use winit::event_loop::EventLoop;
use rotating_cube::App;

fn main() {
    // 이벤트 루프 및 창 생성
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    // event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    // event_loop.set_control_flow(ControlFlow::Wait);

    // Init and run
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);

}
