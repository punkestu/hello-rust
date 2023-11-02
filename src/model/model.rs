#[derive(Debug)]
pub struct Model {
    pub id: u32,
    pub name: String,
}
static mut COUNTER: u32 = 0;
impl Model {
    pub unsafe fn new(name: String) -> Self {
        COUNTER += 1;
        Model {
            id: COUNTER - 1,
            name,
        }
    }
    pub fn clone(&self) -> Self {
        Model {
            id: self.id,
            name: self.name.clone(),
        }
    }
}
