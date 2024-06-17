pub trait ModelTimestamps {
    fn created_at(&mut self) {}
    fn updated_at(&mut self) {}
    fn deleted_at(&mut self) {}
}