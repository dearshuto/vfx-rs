use uuid::Uuid;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ResourceId {
    id: Uuid,
}

impl ResourceId {
    fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct EmitterId {
    id: Uuid,
}

impl EmitterId {
    fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

pub struct ParticleSystem {}

impl ParticleSystem {
    pub fn register_resource(&mut self) -> ResourceId {
        ResourceId::new()
    }

    pub fn create_emitter(&self) -> EmitterId {
        EmitterId::new()
    }

    pub fn destroy_emitter(&self, _id: EmitterId) {}

    pub fn update(&mut self) {}

    pub fn push_command(&self, _id: EmitterId) {}
}
