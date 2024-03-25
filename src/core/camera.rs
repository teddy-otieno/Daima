use nalgebra::{Matrix, Matrix4, Point3, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub pos: Point3<f32>,
    pub target: Vector3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Point3::new(0.0, 0.0, 0.0),
            target: Vector3::new(0.0, 0.0, 0.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn look_matrix(&self) -> Matrix4<f32> {
        dbg!(&self);

        let projection =
            Matrix::new_perspective((45.0_f32).to_radians(), 800.0 / 800.0, 0.1, 100.0);
        let view = Matrix::look_at_rh(&self.pos, &(self.pos + self.front), &self.up);
        projection * view
    }
}
