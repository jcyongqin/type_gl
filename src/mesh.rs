use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_front::context::GraphicsContext;
use luminance_front::tess::{
    Deinterleaved, Interleaved, Mode, Tess, TessBuilder, TessError, TessView,
};
use luminance_front::Backend;
use luminance_glfw::GlfwSurface;

// We derive Semantics automatically and provide the mapping as field attributes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum VertexSemantics {
    // - Reference vertex positions with the "position" variable in vertex shaders.
    // - The underlying representation is [f32; 2], which is a vec2 in GLSL.
    // - The wrapper type you can use to handle such a semantics is VertexPosition.
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    // - Reference vertex colors with the "color" variable in vertex shaders.
    // - The underlying representation is [u8; 3], which is a uvec3 in GLSL.
    // - The wrapper type you can use to handle such a semantics is VertexRGB.
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
    Color,
}

#[derive(Clone, Copy, Debug, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    #[allow(dead_code)]
    position: VertexPosition,

    #[allow(dead_code)]
    #[vertex(normalized = "true")]
    color: VertexRGB,
}

struct MeshRenderer {}

const VERTICES: [Vertex; 3] = [
    Vertex::new(
        VertexPosition::new([-0.5, -0.5]),
        VertexRGB::new([125, 0, 0]),
    ),
    Vertex::new(
        VertexPosition::new([0.5, -0.5]),
        VertexRGB::new([0, 125, 0]),
    ),
    Vertex::new(VertexPosition::new([0., 0.5]), VertexRGB::new([0, 0, 125])),
];

const TRI_DEINT_POS_VERTICES: &[VertexPosition] = &[
    VertexPosition::new([1., -1.]),
    VertexPosition::new([1., 1.]),
    VertexPosition::new([-1., -1.]),
    VertexPosition::new([-1., 1.]),
    VertexPosition::new([0., -0.]),
];

const TRI_DEINT_COLOR_VERTICES: &[VertexRGB] = &[
    VertexRGB::new([0, 255, 0]),
    VertexRGB::new([0, 0, 255]),
    VertexRGB::new([255, 0, 0]),
    VertexRGB::new([255, 51, 255]),
    VertexRGB::new([51, 255, 255]),
    VertexRGB::new([51, 51, 255]),
];

const TRI_INDICES: [u8; 6] = [
    0, 1, 2, // First triangle.
    3, 4, 0, // Second triangle.
];


// TessBuilder<Self::Backend, V, (), W, Deinterleaved>


pub fn get_mesh_2<V>(builder : TessBuilder< V ,>  ) -> Tess<Vertex, u8,  Deinterleaved> {
    let tess1 = surface
        .new_deinterleaved_tess::<Vertex, ()>()
        .set_indices(&TRI_INDICES[..])
        .set_attributes(&TRI_DEINT_POS_VERTICES[..])
        .set_attributes(&TRI_DEINT_COLOR_VERTICES[..])
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

     tess1
}


pub fn get_mesh(surface: &mut GlfwSurface) -> Tess<Vertex> {
    let tess2 = surface
        .new_tess()
        .set_vertices(&VERTICES[..])
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

        tess2
}