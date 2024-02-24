use uuid::Uuid;

#[derive(Debug)]
pub enum Identifier {
    Id(Uuid),
    Email(String),
}
