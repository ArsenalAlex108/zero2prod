use uuid::Uuid;

pub trait UuidGenerator: Send + Sync {
    fn generate_uuid(&self) -> Uuid;
}

pub struct DefaultUuidGenerator;

impl UuidGenerator for DefaultUuidGenerator {
    fn generate_uuid(&self) -> Uuid {
        Uuid::new_v4()
    }
}
