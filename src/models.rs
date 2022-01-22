pub mod ifus;
pub mod products;
pub mod users;

pub trait Record {
    fn id(&self) -> i32;
}
