extern crate rand;

use gl;
use glm::*;
use self::rand::prelude::*;

use gust::mesh::Mesh;

use scene_objects::terrain::*;
use dust::{traits, camera};
use dust::core::{buffer, program, surface, state};

pub struct Grass {
    program: program::Program,
    model: surface::TriangleSurface,
    position_buffer: buffer::VertexBuffer
}

const NO_STRAWS: usize = 128;

impl Grass
{
    pub fn create(gl: &gl::Gl, terrain: &Terrain) -> Result<Grass, traits::Error>
    {
        let positions: Vec<f32> = vec![
            0.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            0.0, 0.3, 0.0,
            1.0, 0.3, 0.0,
            0.0, 0.5, 0.0,
            1.0, 0.5, 0.0,
            0.0, 0.7, 0.0,
            1.0, 0.7, 0.0,
            0.5, 1.0, 0.0,
        ];
        let indices: Vec<u32> = vec![
            0, 1, 2,
            1, 2, 3,
            2, 3, 4,
            3, 4, 5,
            4, 5, 6,
            5, 6, 7,
            6, 7, 8
        ];
        let mut mesh = Mesh::create_indexed(indices, positions)?;

        let program = program::Program::from_resource(gl, "examples/assets/shaders/grass")?;
        let mut model = surface::TriangleSurface::create(gl, &mesh, &program)?;

        let mut position_buffer = buffer::VertexBuffer::create(gl).unwrap();

        program.set_used();
        program.setup_attribute("root_position", 3, 3, 0, 1)?;

        let mut grass = Grass { program, model, position_buffer };
        grass.create_straws(terrain);
        Ok(grass)
    }

    fn random_position(terrain: &Terrain) -> Vec3
    {
        let center = terrain.get_center();
        let x = center.x + (random::<f32>()-0.5) * SIZE;
        let z = center.z + (random::<f32>()-0.5) * SIZE;
        let height = terrain.get_height_at(x, z);
        if height < 0.1
        {
            return Grass::random_position(terrain)
        }
        vec3(x, height, z)
    }

    pub fn create_straws(&mut self, terrain: &Terrain)
    {
        let mut root_positions = Vec::new();
        for i in 0..NO_STRAWS {
            let p = Grass::random_position(terrain);
            root_positions.push(p.x);
            root_positions.push(p.y);
            root_positions.push(p.z);
        }

        self.position_buffer.fill_with(&root_positions);
    }

    pub fn render(&self, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.program.cull(state::CullType::NONE);

        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;

        self.model.render_instances(NO_STRAWS)?;
        Ok(())
    }
}