#[allow(unused_variables)]
pub trait Device {
    fn set_context(&mut self, idx: u16, value: u16) {}
    fn send(&mut self) {}

    fn read_context(&self, idx: u16) -> u16 { 0 }
    fn has_response(&self) -> bool { false }
    fn receive(&mut self);
}