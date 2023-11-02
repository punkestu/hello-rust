use crate::model::{Error, Model};
use crate::repo::{ModelCreateParam, Repo};
use std::io::{stdin, stdout, Write};

pub struct Service {
    pub service_repo: Repo,
}

impl Service {
    pub fn new(service_repo: Repo) -> Self {
        Service {
            service_repo: service_repo,
        }
    }
    pub fn get_data(&self) -> &Vec<Model> {
        self.service_repo.get()
    }
    pub fn create_data(&mut self) -> Result<bool, Error> {
        print!("insert new user name: ");
        let _ = stdout().flush();
        let mut name = String::new();
        let _ = stdin().read_line(&mut name);
        self.service_repo.create(ModelCreateParam {
            name: name.trim_end().to_string(),
        })?;
        Ok(true)
    }
}
