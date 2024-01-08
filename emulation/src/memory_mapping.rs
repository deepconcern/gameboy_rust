use crate::memory_component::{MemoryComponent, MemoryError, UnimplementedMemory};

pub struct MemoryMapping {
    components: Vec<Box<dyn MemoryComponent>>,
    memory_mapping: Vec<usize>,
}

impl MemoryMapping {
    pub fn new() -> Self {
        let mut memory_mapping = MemoryMapping {
            components: vec![],
            memory_mapping: vec![0; u16::MAX as usize],
        };

        memory_mapping.register_component(Box::new(UnimplementedMemory::new()));

        memory_mapping
    }

    pub fn read(&self, location: u16) -> Result<u8, MemoryError> {
        let component_index = self.memory_mapping[location as usize];

        let component = self.components.get(component_index).unwrap();

        component.read(location)
    }

    pub fn register_component(&mut self, component: Box<dyn MemoryComponent>) -> &mut Self {
        let component_index = self.components.len();

        self.components.push(component);

        for location in &self.components[component_index].mapped_locations() {
            self.memory_mapping[*location as usize] = component_index;
        }

        self
    }

    pub fn write(&mut self, location: u16, value: u8) -> Result<(), MemoryError> {
        let component_index = self.memory_mapping[location as usize];

        let component = self.components.get_mut(component_index).unwrap();

        component.write(location, value)
    }
}
