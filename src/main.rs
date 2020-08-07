use cgmath::{perspective, EuclideanSpace, Matrix4, Point3, Rad, Vector3};
use glfw::{Action, Context as _, Key, WindowEvent};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_front::context::GraphicsContext;
use luminance_front::pipeline::{Pipeline, PipelineState, Render};
use luminance_front::render_gate::RenderGate;
use luminance_front::render_state::RenderState;
use luminance_front::shader::{ProgramInterface, Uniform, UniformInterface};
use luminance_front::shading_gate::ShadingGate;
use luminance_front::tess::{Interleaved, Mode, Tess, TessError};
use luminance_front::Backend;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;
use std::process::exit;
use std::time::Instant;
// use try_guard::verify;
// use wavefront_obj::obj;

mod mesh;
mod shader;
// We get the shader at compile time from local files
// const VS: &'static str = include_str!("simple-vs.glsl");
// const FS: &'static str = include_str!("simple-fs.glsl");

// Vertex semantics. Those are needed to instruct the GPU how to select vertex’s attributes from
// the memory we fill at render time, in shaders. You don’t have to worry about them; just keep in
// mind they’re mandatory and act as “protocol” between GPU’s memory regions and shaders.
//

const FOVY: Rad<f32> = Rad(std::f32::consts::FRAC_PI_2);
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 10.;

fn main() {
    // First thing first: we create a new surface to render to and get events from.
    let dim = WindowDim::Windowed {
        width: 96,
        height: 54,
    };
    let mut surface = GlfwSurface::new_gl33(
        "Hello, world; from OpenGL 3.3!",
        WindowOpt::default().set_dim(dim),
    );
    // .expect("GLFW surface creation");

    if let Err(e) = surface {
        eprintln!("cannot create graphics surface:\n{}", e);
        exit(1);
    } else {
        eprintln!("graphics surface created");
        main_loop(surface.expect("GLFW surface creation"));
    }

}

fn main_loop(mut surface: GlfwSurface) {
    // //   let mut demo = TessMethod::Direct;
    // resize window
    let mut resize = true;

    // let triangle =;

    let mut program = shader::get_shader(&mut surface);

    // let projection = perspective(FOVY, width as f32 / height as f32, Z_NEAR, Z_FAR);
    // let view =
    //     Matrix4::<f32>::look_at(Point3::new(2., 2., 2.), Point3::origin(), Vector3::unit_y());
    let color = [0.8, 1., 0.6, 1.];

    let mut back_buffer = surface.back_buffer().unwrap();
    'app: loop {
        // For all the events on the surface.
        surface.window.glfw.poll_events();
        for (_, event) in surface.events_rx.try_iter() {
            match event {
                // If we close the window or press escape, quit the main loop (i.e. quit the application).
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'app
                }

                // // If we hit the spacebar, change the kind of tessellation.
                // WindowEvent::Key(Key::Space, code, Action::Release, _) => {
                // println!("now key {:?}", code);
                // }
                WindowEvent::Key(key, code, Action::Release, _) => {
                    println!("now key {:?} - {:?}", key, code);
                }

                // Handle window resizing.
                WindowEvent::FramebufferSize(..) => {
                    resize = true;
                }
                _ => (),
            }
        }

        if resize {
            // Simply ask another backbuffer at the right dimension (no allocation / reallocation).
            back_buffer = surface.back_buffer().unwrap();
            let [width, height] = back_buffer.size();
            resize = false;
        }

        //     let t = start_t.elapsed().as_millis() as f32 * 1e-3;
        //     let color = [t.cos(), t.sin(), 0.5, 1.];

        // rendering code goes here
        let render_able =
            |_: ProgramInterface, _: &(), mut rdr_gate: RenderGate| -> Result<(), _> {
                rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                    // …
                    tess_gate.render( &mesh::get_mesh1(surface.new_deinterleaved_tess()))
                })
            };

        let render_pass = |_: Pipeline, mut shd_gate: ShadingGate| -> Result<(), _> {
            let ret = shd_gate.shade(&mut program, render_able);

            ret
        };

        // Create a new dynamic pipeline that will render to the back buffer and must clear it with
        // pitch black prior to do any render to it.
        let render = surface
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default().set_clear_color(color),
                render_pass,
            )
            .assume();

        // Finally, swap the backbuffer chains
        if render.is_ok() {
            surface.window.swap_buffers();
        } else {
            break 'app;
        }
    }
}
