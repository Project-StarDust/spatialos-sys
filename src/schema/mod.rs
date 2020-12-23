// use crate::ffi::*;

use crate::ffi::*;

pub mod object;
pub use object::Object;

pub type EntityId = i64;
pub type FieldId = u32;

pub const MAP_KEY_FIELD_ID: u32 = SCHEMA_MAP_KEY_FIELD_ID;
pub const MAP_VALUE_FIELD_ID: u32 = SCHEMA_MAP_VALUE_FIELD_ID;

#[derive(Debug, Clone)]
pub struct ComponentData {
    inner: Box<Schema_ComponentData>,
}

#[derive(Debug, Clone)]
pub struct ComponentUpdate {
    inner: Box<Schema_ComponentUpdate>,
}

impl ComponentData {
    pub fn new() -> Self {
        let inner = unsafe { Box::from_raw(Schema_CreateComponentData()) };
        Self { inner }
    }

    pub fn get_fields(&mut self) -> Object {
        Object::from(unsafe {
            Schema_GetComponentDataFields(&mut *self.inner as *mut Schema_ComponentData)
        })
    }
}

impl ComponentUpdate {
    pub fn new() -> Self {
        let inner = unsafe { Box::from_raw(Schema_CreateComponentUpdate()) };
        Self { inner }
    }

    pub fn get_fields(&mut self) -> Object {
        Object::from(unsafe {
            Schema_GetComponentUpdateFields(&mut *self.inner as *mut Schema_ComponentUpdate)
        })
    }
}

impl From<*mut Schema_ComponentData> for ComponentData {
    fn from(inner: *mut Schema_ComponentData) -> Self {
        let inner = unsafe { Box::from_raw(inner) };
        Self { inner }
    }
}

impl From<ComponentData> for *mut Schema_ComponentData {
    fn from(data: ComponentData) -> Self {
        Box::into_raw(data.inner)
    }
}

impl From<*mut Schema_ComponentUpdate> for ComponentUpdate {
    fn from(inner: *mut Schema_ComponentUpdate) -> Self {
        let inner = unsafe { Box::from_raw(inner) };
        Self { inner }
    }
}

impl From<ComponentUpdate> for *mut Schema_ComponentUpdate {
    fn from(data: ComponentUpdate) -> Self {
        Box::into_raw(data.inner)
    }
}
