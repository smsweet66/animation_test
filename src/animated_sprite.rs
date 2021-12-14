use std::borrow::Borrow;
use glium::*;
use glium::index::PrimitiveType::TrianglesList;
use image::io::Reader;
use nalgebra_glm::*;
use glium::texture::SrgbTexture2d;

pub(crate) struct AnimatedSprite
{
    rows: f32,
    cols: f32,

    vb: VertexBuffer<Vertex>,
    ib: IndexBuffer<u32>,

    texture: SrgbTexture2d
}

impl AnimatedSprite
{
    pub fn new(sprite_location: &str, rows: f32, cols: f32, display: &Display) -> Self
    {
        let image = Reader::open(sprite_location).unwrap().decode().unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = texture::SrgbTexture2d::new(display, image).unwrap();
        implement_vertex!(Vertex, position, tex_coord);
        let vertices = [
            Vertex::from_2f(0.0, image_dimensions.1 as f32/rows, 0.0, 1.0 - 1.0/(rows as f32)),
            Vertex::from_2f(0.0, 0.0, 0.0, 0.0),
            Vertex::from_2f(image_dimensions.0 as f32/cols, image_dimensions.1 as f32/rows, 1.0/(cols as f32), 1.0 - 1.0/(rows as f32)),
            Vertex::from_2f(image_dimensions.0 as f32/cols, 0.0, 1.0/(cols as f32), 0.0)
        ];
        return Self {
            rows, cols,
            vb: VertexBuffer::new(display, &vertices).unwrap(),
            ib: IndexBuffer::new(display, TrianglesList, &[0, 1, 2, 1, 2, 3]).unwrap(),
            texture
        }
    }

    pub fn draw(&self, target: &mut Frame, shader_program: &Program, row: u32, col: u32, x_pos: f32, y_pos: f32, proj: &Mat4)
    {
        let u_proj: [[f32; 4]; 4] = (*proj).into();
        let mut model = Mat4::identity();
        model = model * translate(&model, Vec3::new(x_pos, y_pos, 0.0).borrow());
        let u_model: [[f32; 4]; 4] = model.into();
        let u_texture_translate = [col as f32/self.cols, -(row as f32/self.rows)];
        let uniforms = uniform! { u_proj: u_proj, u_model: u_model, u_Texture: &self.texture, u_texture_translate: u_texture_translate };
        target.draw(&self.vb, &self.ib, shader_program, &uniforms, &DrawParameters::default()).unwrap();
    }
}

#[derive(Copy, Clone)]
struct Vertex
{
    position: [f32; 4],
    tex_coord: [f32; 2]
}

impl Vertex
{
    pub fn from_4f(p1: f32, p2: f32, p3: f32, p4: f32, t1: f32, t2: f32) -> Self
    { return Self { position: [p1, p2, p3, p4], tex_coord: [t1, t2] } }

    pub fn from_3f(p1: f32, p2: f32, p3: f32, t1: f32, t2: f32) -> Self
    { return Self { position: [p1, p2, p3, 1.0], tex_coord: [t1, t2] } }

    pub fn from_2f(p1: f32, p2: f32, t1: f32, t2: f32) -> Self
    { return Self { position: [p1, p2, 0.0, 1.0], tex_coord: [t1, t2] } }
}