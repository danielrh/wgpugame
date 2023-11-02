// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}
impl Default for CameraUniform {
    fn default() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }
}
impl CameraUniform {
    pub fn update_view_proj(&mut self, width: f32, height: f32) {
        self.view_proj[0][0] = height / width.max(1.0);
        eprintln!("View Proj {:?}", self.view_proj);
    }
}
 
