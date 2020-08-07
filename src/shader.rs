use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_front::context::GraphicsContext;
use luminance_front::shader::Program;
use luminance_front::shader::Uniform;
use luminance_glfw::GlfwSurface;

use super::mesh;
#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
    #[uniform(unbound)]
    projection: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    view: Uniform<[[f32; 4]; 4]>,
}

// const VS_STR_ :String  =.to_string() + ;
const VS_STR: &str = concat!("//", include_str!("../shader/simple-vs.glsl.vs"));
const FS_STR: &str = concat!("//", include_str!("../shader/simple-fs.glsl.fs"));

pub fn get_shader(surface: &mut GlfwSurface) -> Program<mesh::VertexSemantics, (), ()> {
    surface
        .new_shader_program::<mesh::VertexSemantics, (), ()>()
        .from_strings(VS_STR, None, None, FS_STR)
        .unwrap()
        .ignore_warnings()
}
