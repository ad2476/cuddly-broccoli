use crate::rendergl::{self, Program};
use crate::resources::ResourceLoader;
use crate::shape::ShapeGL;
use tobj;

pub struct SceneModel {
    pub shapegl: ShapeGL,
    pub material_id: Option<usize>,
}

impl SceneModel {
    pub fn new(shapegl: ShapeGL, material_id: Option<usize>) -> SceneModel {
        SceneModel {
            shapegl,
            material_id,
        }
    }
}

impl From<&tobj::Model> for SceneModel {
    fn from(other: &tobj::Model) -> Self {
        SceneModel {
            shapegl: ShapeGL::from_mesh(&other.mesh),
            material_id: other.mesh.material_id,
        }
    }
}

pub struct MaterialShader {
    pub program: Program,
}

impl MaterialShader {
    pub fn from_res(
        res: &ResourceLoader,
        name: &str,
    ) -> Result<MaterialShader, rendergl::shader::Error> {
        let program = Program::from_res(res, name)?;
        Ok(MaterialShader { program })
    }

    pub fn apply_material(
        &self,
        material: &tobj::Material,
    ) -> Result<(), rendergl::uniform::Error> {
        self.program.set_uniform("cDiffuse", &material.diffuse)?;
        //self.program.set_uniform("cAmbient", &material.ambient)?;
        //self.program.set_uniform("cSpecular", &material.specular)?;
        //self.program.set_uniform("shininess", &material.shininess)?;
        Ok(())
    }
}
