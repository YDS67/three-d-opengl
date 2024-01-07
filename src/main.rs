#![windows_subsystem = "windows"]

use three_d::Event;
use three_d::Key;
use three_d::SurfaceSettings;
use three_d::core::{
    degrees, radians, vec2, vec3, vec4, ClearState, Context, Mat4, Program, RenderStates, VertexBuffer, ElementBuffer, Texture2D, CpuTexture, TextureData};
use three_d::window::{FrameOutput, Window, WindowSettings};
use three_d_asset::Camera;

use std::thread::sleep;
use std::time::Duration;

const FT_DESIRED: f32 = 0.01666666666667;

mod assets;

fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Three-D example, WASD to rotate, Esc to quit".to_string(),
        max_size: Some((900, 720)),
        min_size: (900, 720),
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
        include_str!("triangle.vert"),
        include_str!("triangle.frag"),
    )
    .unwrap();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

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

        camera.set_viewport(frame_input.viewport);

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
                    anglex += 1.0;
                }
                if key_d {
                    anglex -= 1.0;
                }
                if key_w {
                    angley += 1.0;
                }
                if key_s {
                    angley -= 1.0;
                }
                program.use_uniform("model", Mat4::from_angle_y(radians(anglex*0.05))*Mat4::from_angle_z(radians(angley*0.05)));
                program.use_uniform("viewProjection", camera.projection() * camera.view());
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

