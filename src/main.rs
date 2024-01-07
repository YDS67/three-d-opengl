#![windows_subsystem = "windows"]

use three_d::renderer::{Event, Key};
use three_d::core::{vec2, vec3, vec4, ClearState, Context, Mat4, Program, RenderStates, VertexBuffer, ElementBuffer, Texture2D, CpuTexture, TextureData};
use three_d::window::{FrameOutput, Window, WindowSettings, SurfaceSettings};

use glam;
use std::thread::sleep;
use std::time::Duration;

const WIDTH0: u32 = 900;
const HEIGHT0: u32 = 720;
const FT_DESIRED: f32 = 0.01666666666667;

mod assets;

fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Three-D example, WASD to rotate, Esc to quit".to_string(),
        max_size: Some((WIDTH0, HEIGHT0)),
        min_size: (WIDTH0, HEIGHT0),
        surface_settings: SurfaceSettings {
            vsync: false,
            depth_buffer: 0,
            stencil_buffer: 0,
            multisamples: 0,
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context: Context = window.gl();

    // Define triangle vertices and colors
    let positions = VertexBuffer::new_with_data(
        &context,
        &[
            vec3(0.5, -0.5, 0.0),  // top right
            vec3(0.5, 0.5, 0.0), // bottom right
            vec3(-0.5, 0.5, 0.0),   // bottom left
            vec3(-0.5, -0.5, 0.0),   // top left
        ],
    );
    let colors = VertexBuffer::new_with_data(
        &context,
        &[
            vec4(1.0, 0.0, 0.0, 1.0),   // top right
            vec4(1.0, 1.0, 0.0, 1.0), // bottom right
            vec4(0.0, 0.0, 1.0, 1.0),  // bottom left
            vec4(0.0, 1.0, 0.0, 1.0),  // top left
        ],
    );

    let uvs = VertexBuffer::new_with_data(
        &context,
        &[
            vec2(1.0, 0.0),  // top right
            vec2(1.0, 1.0), // bottom right
            vec2(0.0, 1.0),   // bottom left
            vec2(0.0, 0.0),   // top left
        ],
    );

    let indices: &[u32] = &[
        0, 1, 3,
        1, 2, 3,
    ];

    let elements = ElementBuffer::new_with_data(
        &context,
        indices,
    );

    let assets = assets::Assets::load();

    let image = CpuTexture{
        name: format!("tex"),
        width: assets.width,
        height: assets.height,
        data: TextureData::RgbaU8(assets.tex),
        ..Default::default()
    };

    let texture = Texture2D::new(&context, &image);

    let program = Program::from_source(
        &context,
        include_str!("vertex.glsl"),
        include_str!("fragment.glsl"),
    )
    .unwrap();

    let mut anglex = 0.0;
    let mut angley = 0.0;
    let mut key_a = false;
    let mut key_d = false;
    let mut key_w = false;
    let mut key_s = false;
    let mut should_close = false;

    let mut time_state = TimeState::init();

    window.render_loop(move |frame_input| {
        time_state.frame_time();
        time_state.show_data();
        let proj = Proj::new(anglex, angley, WIDTH0 as f32/HEIGHT0 as f32);

        for event in frame_input.events.iter() {
            match event {
                Event::KeyPress { kind: Key::Escape, ..} => {
                    should_close = true
                }
                Event::KeyPress { kind: Key::A, ..} => {
                    key_a = true
                }
                Event::KeyPress { kind: Key::D, ..} => {
                    key_d = true
                }
                Event::KeyPress { kind: Key::W, ..} => {
                    key_w = true
                }
                Event::KeyPress { kind: Key::S, ..} => {
                    key_s = true
                }
                Event::KeyRelease { kind: Key::A, ..} => {
                    key_a = false
                }
                Event::KeyRelease { kind: Key::D, ..} => {
                    key_d = false
                }
                Event::KeyRelease { kind: Key::W, ..} => {
                    key_w = false
                }
                Event::KeyRelease { kind: Key::S, ..} => {
                    key_s = false
                }
                _ => {}
            }
        }

        frame_input
            .screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .write(|| {
                if key_a {
                    anglex += 5.0*time_state.frame_time;
                }
                if key_d {
                    anglex -= 5.0*time_state.frame_time;
                }
                if key_w {
                    angley += 5.0*time_state.frame_time;
                }
                if key_s {
                    angley -= 5.0*time_state.frame_time;
                }

                program.use_uniform("viewProjection", proj.mvp);
                program.use_vertex_attribute("position", &positions);
                program.use_vertex_attribute("color", &colors);
                program.use_vertex_attribute("uv", &uvs);
                program.use_texture("tex", &texture);
                program.draw_elements(
                    RenderStates::default(),
                    frame_input.viewport,
                    &elements,
                );
            });

        time_state.last_frame = Some(std::time::Instant::now()).unwrap();

        time_state.frame_count += 1;

        if should_close {
            return FrameOutput {exit: true, swap_buffers: false, wait_next_event: false}
        } else {
            return FrameOutput::default()
        }
    });
}

struct TimeState {
    last_frame: std::time::Instant,
    frame_time: f32,
    frame_count: u128,
    fps: i32,
}

impl TimeState {
    fn init() -> TimeState {
        TimeState {
            last_frame: Some(std::time::Instant::now()).unwrap(),
            frame_time: 1.0 / 60.0,
            frame_count: 0,
            fps: 60,
        }
    }

    fn frame_time(&mut self) {
        self.frame_time = self.last_frame.elapsed().as_secs_f32();
        if self.frame_time < FT_DESIRED {
            sleep(Duration::from_secs_f32(
                FT_DESIRED - self.frame_time,
            ));
        }
        self.frame_time = self.last_frame.elapsed().as_secs_f32();
        self.fps = (1. / self.frame_time).floor() as i32 + 1;
    }
    
    fn show_data(&mut self) {
        if self.frame_count.overflowing_rem(60).0 == 0 {
            println!("FPS: {}, Frames: {}", 
                self.fps, self.frame_count);
        }
    }
}

struct Proj {
    mvp: Mat4,
}

impl Proj {
    fn new(angle_x: f32, angle_y: f32, asp: f32) -> Proj {
        let proj = glam::Mat4::perspective_rh_gl(
            45.0/180.0*3.14159,
            asp,
            0.1,
            10.0,
        );

        let rotx = glam::Mat4::from_rotation_y(angle_x);
        let roty = glam::Mat4::from_rotation_z(angle_y);

        let view = glam::Mat4::look_to_rh(
            glam::vec3(0.0, 0.0, 2.0),
            glam::vec3(0.0, 0.0, -1.0),
            glam::vec3(0.0, 1.0, 0.0),
        );
        let mvp_glam = proj * view * rotx * roty;
        let mvp = Mat4{
            x: vec4(mvp_glam.x_axis.x, mvp_glam.x_axis.y, mvp_glam.x_axis.z, mvp_glam.x_axis.w),
            y: vec4(mvp_glam.y_axis.x, mvp_glam.y_axis.y, mvp_glam.y_axis.z, mvp_glam.y_axis.w),
            z: vec4(mvp_glam.z_axis.x, mvp_glam.z_axis.y, mvp_glam.z_axis.z, mvp_glam.z_axis.w),
            w: vec4(mvp_glam.w_axis.x, mvp_glam.w_axis.y, mvp_glam.w_axis.z, mvp_glam.w_axis.w),
        };

        Proj { mvp }
    }
}