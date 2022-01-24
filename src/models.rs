pub mod ifus;
pub mod products;
pub mod users;

pub trait Record {
    fn id(&self) -> String;
    fn kind(&self) -> String;
}
