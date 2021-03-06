use std::f32::consts::FRAC_PI_2;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Instant;

use cgmath::{Matrix4, PerspectiveFov, Point3, Rad, SquareMatrix, Vector3};
use glow::{Context, HasContext};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::{event_loop::EventLoop, window::Window, ContextWrapper, PossiblyCurrent};

use crate::camera::Camera;
use crate::input_handler::InputHandler;
use crate::world::World;
#[derive(Clone, Debug)]
pub struct GlContext {
    gl: Rc<Context>,
}

impl GlContext {
    pub fn new(gl: Context) -> Self {
        Self { gl: Rc::new(gl) }
    }
}

impl Deref for GlContext {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

pub struct WindowHandler {
    gl: GlContext,
    window: ContextWrapper<PossiblyCurrent, Window>,
    event_loop: EventLoop<()>,
}

#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

#[macro_export]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

impl WindowHandler {
    pub fn new(width: i32, height: i32, vsync: bool) -> Self {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(env!("CARGO_BIN_NAME"))
            .with_inner_size(glutin::dpi::LogicalSize::new(width as f32, height as f32));
        unsafe {
            let window = glutin::ContextBuilder::new()
                .with_vsync(vsync)
                .with_depth_buffer(24)
                .with_double_buffer(Some(true))
                .with_multisampling(2)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();
            let gl = GlContext::new(glow::Context::from_loader_function(|s| {
                window.get_proc_address(s) as *const _
            }));
            Self {
                gl,
                window,
                event_loop,
            }
        }
    }

    pub fn run(self) {
        let WindowHandler {
            gl,
            window,
            event_loop,
        } = self;

        {
            let mut world = World::new(&gl);
            let mut input_handler = InputHandler::new();
            unsafe {
                gl.enable(glow::DEPTH_TEST);
                gl.enable(glow::BLEND);
                gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
                gl.clear_color(0.0, 0.0, 0.0, 1.0);
            }

            let perspective = PerspectiveFov::<f32> {
                fovy: Rad(90.0),
                aspect: window.window().inner_size().width as f32
                    / window.window().inner_size().height as f32,
                near: 0.1,
                far: 15_000.0,
            };

            let eye = Point3::<f32>::new(0.0, 0.0, 0.0);
            let direction = Vector3::<f32>::new(0.0, 0.0, -1.0);
            let up = Vector3::<f32>::new(0.0, 1.0, 0.0);

            let mut camera = Camera {
                eye,
                direction,
                up,
                perspective,
                view: Matrix4::identity(),
            };

            let mut updates = 0;
            let mut renders = 0;
            let mut last_update = Instant::now();
            let update_timer = Instant::now();
            let second = 1_000_000_000;
            let tick_rate: u64 = 128;
            let tick_time: u64 = second as u64 / tick_rate;
            let mut tick_timer = 0;
            let mut cumulative_time: u128 = 0;

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;
                window.window().set_cursor_visible(false);
                match event {
                    Event::LoopDestroyed => (),
                    Event::MainEventsCleared => {
                        while cumulative_time < update_timer.elapsed().as_nanos() {
                            cumulative_time += tick_time as u128;
                            world.update(input_handler.get_key_state());
                            updates += 1;
                        }

                        if tick_timer < update_timer.elapsed().as_nanos() {
                            tick_timer += second;
                            println!("Updates: {} Renders: {}", updates, renders);
                            //println!("{:#?}", input_handler.get_key_state());
                            updates = 0;
                            renders = 0;
                            //gl_get_error(&gl);
                        }
                        window.window().request_redraw();
                    }
                    Event::RedrawRequested(_) => {
                        unsafe {
                            gl.clear(glow::DEPTH_BUFFER_BIT | glow::COLOR_BUFFER_BIT);
                        }

                        //let cam_per: [f32; 16] = *perspective.mul(view).as_ref();
                        let now = Instant::now();
                        world.render(&now.duration_since(last_update).as_secs_f32(), &mut camera);
                        window.swap_buffers().unwrap();
                        last_update = now;
                        renders += 1;
                    }
                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::KeyboardInput { input, .. } => {
                            input_handler.key_pressed(input);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            window.resize(**new_inner_size);
                        }
                        WindowEvent::Resized(size) => unsafe {
                            camera.perspective.fovy = Rad(FRAC_PI_2);
                            camera.perspective.aspect = size.width as f32 / size.height as f32;
                            gl.viewport(0, 0, size.width as i32, size.height as i32);
                        },
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => (),
                    },
                    _ => (),
                }
            });
        }
    }
}
