// use crate::ffi::*;

use crate::ffi::*;

pub type EntityId = i64;
pub type FieldId = u32;

pub struct ComponentData {
    inner: Box<Schema_ComponentData>,
}

pub struct ComponentUpdate {
    inner: Box<Schema_ComponentUpdate>,
}

pub struct Object {
    inner: Box<Schema_Object>,
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


impl Object {
    pub fn add_bool(&mut self, field_id: FieldId, value: bool) {
        unsafe {
            Schema_AddBool(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value as u8,
            )
        }
    }

    pub fn get_bool(&mut self, field_id: FieldId) -> bool {
        let result = unsafe { Schema_GetBool(&mut *self.inner as *mut Schema_Object, field_id) };
        result > 0
    }

    pub fn add_double(&mut self, field_id: FieldId, value: f64) {
        unsafe { Schema_AddDouble(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn get_double(&mut self, field_id: FieldId) -> f64 {
        unsafe { Schema_GetDouble(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_entity_id(&mut self, field_id: FieldId, value: EntityId) {
        unsafe { Schema_AddEntityId(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn get_entity_id(&mut self, field_id: FieldId) -> EntityId {
        unsafe { Schema_GetEntityId(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_fixed32(&mut self, field_id: FieldId, value: u32) {
        unsafe { Schema_AddFixed32(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }
    pub fn get_fixed32(&mut self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetFixed32(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_fixed64(&mut self, field_id: FieldId, value: u64) {
        unsafe { Schema_AddFixed64(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }
    pub fn get_fixed64(&mut self, field_id: FieldId) -> u64 {
        unsafe { Schema_GetFixed64(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_float(&mut self, field_id: FieldId, value: f32) {
        unsafe { Schema_AddFloat(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }
    pub fn get_float(&mut self, field_id: FieldId) -> f32 {
        unsafe { Schema_GetFloat(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_int32(&mut self, field_id: FieldId, value: i32) {
        unsafe { Schema_AddInt32(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }
    pub fn get_int32(&mut self, field_id: FieldId) -> i32 {
        unsafe { Schema_GetInt32(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_int64(&mut self, field_id: FieldId, value: i64) {
        unsafe { Schema_AddInt64(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }
    pub fn get_int64(&mut self, field_id: FieldId) -> i64 {
        unsafe { Schema_GetInt64(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_sfixed32(&mut self, field_id: FieldId, value: i32) {
        unsafe { Schema_AddSfixed32(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn get_sfixed32(&mut self, field_id: FieldId) -> i32 {
        unsafe { Schema_GetSfixed32(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_sfixed64(&mut self, field_id: FieldId, value: i64) {
        unsafe { Schema_AddSfixed64(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }
    pub fn get_sfixed64(&mut self, field_id: FieldId) -> i64 {
        unsafe { Schema_GetSfixed64(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_sint32(&mut self, field_id: FieldId, value: i32) {
        unsafe { Schema_AddSint32(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn get_sint32(&mut self, field_id: FieldId) -> i32 {
        unsafe { Schema_GetSint32(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_sint64(&mut self, field_id: FieldId, value: i64) {
        unsafe { Schema_AddSint64(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn get_sint64(&mut self, field_id: FieldId) -> i64 {
        unsafe { Schema_GetSint64(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_uint32(&mut self, field_id: FieldId, value: u32) {
        unsafe { Schema_AddUint32(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn get_uint32(&mut self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetUint32(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_uint64(&mut self, field_id: FieldId, value: u64) {
        unsafe { Schema_AddUint64(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }
    pub fn get_uint64(&mut self, field_id: FieldId) -> u64 {
        unsafe { Schema_GetUint64(&mut *self.inner as *mut Schema_Object, field_id) }
    }

    pub fn add_object(&mut self, field_id: FieldId) -> Self {
        let inner = unsafe {
            Box::from_raw(Schema_AddObject(
                &mut *self.inner as *mut Schema_Object,
                field_id,
            ))
        };
        Self { inner }
    }
    pub fn get_object(&mut self, field_id: FieldId) -> Self {
        let inner = unsafe {
            Box::from_raw(Schema_GetObject(
                &mut *self.inner as *mut Schema_Object,
                field_id,
            ))
        };
        Self { inner }
    }

    pub fn get_object_count(&mut self, field_id: FieldId) -> u32 {
        unsafe {
            Schema_GetObjectCount(
                &mut *self.inner as *mut Schema_Object,
                field_id,
            )
        }
    }
}

impl From<*mut Schema_Object> for Object {
    fn from(inner: *mut Schema_Object) -> Self {
        let inner = unsafe { Box::from_raw(inner) };
        Self { inner }
    }
}
