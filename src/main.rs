mod model;
mod repo;
mod service;

fn main() {
    let r = repo::Repo::new();
    let mut s = service::Service::new(r);
    match s.create_data() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };
    println!("{:?}", s.get_data());
}
