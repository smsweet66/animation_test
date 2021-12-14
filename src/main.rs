mod shader;
mod animated_sprite;

extern crate glium;
extern crate image;

use glium::*;
use nalgebra_glm::*;
use crate::animated_sprite::AnimatedSprite;
use crate::glutin::GlProfile;
use crate::glutin::window::Fullscreen;
use crate::shader::gen_shader_program;

fn main()
{
    let screen_width = 2560.0;
    let screen_height = 1440.0;
    let hertz = 144.0;

    let rows = 2.0;
    let cols = 8.0;

    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_fullscreen(Option::Some(Fullscreen::Borderless(Option::None)));
    let context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_hardware_acceleration(Option::Some(true))
        .with_gl_profile(GlProfile::Core);
    let display = Display::new(window, context, &event_loop).unwrap();
    let shader_program = gen_shader_program(&display);

    let proj: Mat4 = ortho(0.0, screen_width as f32, 0.0, screen_height as f32, -1.0, 1.0);

    let sprite = AnimatedSprite::new("resources/sprites/test.png",
        rows, cols, &display);

    let mut col = 0;
    event_loop.run(move |event, _, control_flow|
    {
        match event
        {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_secs_f32(1.0/hertz);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        sprite.draw(&mut target, &shader_program, 1, (col * 12)/(hertz as u32), screen_width/2.0, screen_height/2.0, &proj);

        col = (col + 1)%((cols*hertz) as u32);
        target.finish().unwrap();
    });
}