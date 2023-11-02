use crate::model::{Error, Model, ERR_NAME_IS_USED};

pub struct Repo {
    data: Vec<Model>,
}

pub struct ModelCreateParam {
    pub name: String,
}

impl Repo {
    pub fn new() -> Self {
        unsafe {
            let repo = Repo {
                data: vec![Model::new("bima".to_string())],
            };
            repo
        }
    }
    pub fn get(&self) -> &Vec<Model> {
        &self.data
    }
    pub fn create(&mut self, param: ModelCreateParam) -> Result<Model, Error> {
        if param.name == "bima" {
            return Err(ERR_NAME_IS_USED);
        }
        unsafe {
            let new_model = Model::new(param.name);
            self.data.push(new_model.clone());
            Ok(new_model)
        }
    }
}
