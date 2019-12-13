pub trait GravityAffected {
    fn fall(&mut self, step: f32);
}
