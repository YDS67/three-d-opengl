#![windows_subsystem = "windows"]

use three_d::Event;
use three_d::Key;
use three_d::SurfaceSettings;
use three_d::core::{
    degrees, radians, vec2, vec3, ClearState, Context, Mat4, Program, RenderStates, VertexBuffer, ElementBuffer, Srgba, Texture2D, CpuTexture, TextureData};
use three_d::window::{FrameOutput, Window, WindowSettings};
use three_d_asset::Camera;

mod assets;

fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Quand + Texture example".to_string(),
        min_size: (1280, 720),
        surface_settings: SurfaceSettings {
            vsync: true,
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
            Srgba::RED.to_linear_srgb(),   // top right
            Srgba::RED.to_linear_srgb() + Srgba::GREEN.to_linear_srgb(), // bottom right
            Srgba::BLUE.to_linear_srgb(),  // bottom left
            Srgba::GREEN.to_linear_srgb(),  // top left
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

    let mut image = CpuTexture{
        name: format!("tex"),
        width: assets.width,
        height: assets.height,
        data: TextureData::RgbaU8(assets.tex),
        ..Default::default()
    };

    image.data.to_linear_srgb();

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

    window.render_loop(move |frame_input| {
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

        if should_close {
            FrameOutput {exit: true, swap_buffers: false, wait_next_event: false}
        } else {
            FrameOutput::default()
        }
    });
}
