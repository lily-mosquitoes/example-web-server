pub mod ifus;
pub mod products;

pub trait Record {
    fn id(&self) -> i32;
}
