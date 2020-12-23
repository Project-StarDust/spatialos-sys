use super::Object;
use crate::ffi::*;
use crate::schema::{EntityId, FieldId};

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

    pub fn add_double(&mut self, field_id: FieldId, value: f64) {
        unsafe { Schema_AddDouble(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn add_entity_id(&mut self, field_id: FieldId, value: EntityId) {
        unsafe { Schema_AddEntityId(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn add_fixed32(&mut self, field_id: FieldId, value: u32) {
        unsafe { Schema_AddFixed32(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn add_fixed64(&mut self, field_id: FieldId, value: u64) {
        unsafe { Schema_AddFixed64(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }
    pub fn add_float(&mut self, field_id: FieldId, value: f32) {
        unsafe { Schema_AddFloat(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn add_int32(&mut self, field_id: FieldId, value: i32) {
        unsafe { Schema_AddInt32(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn add_int64(&mut self, field_id: FieldId, value: i64) {
        unsafe { Schema_AddInt64(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn add_sfixed32(&mut self, field_id: FieldId, value: i32) {
        unsafe { Schema_AddSfixed32(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn add_sfixed64(&mut self, field_id: FieldId, value: i64) {
        unsafe { Schema_AddSfixed64(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn add_sint32(&mut self, field_id: FieldId, value: i32) {
        unsafe { Schema_AddSint32(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn add_sint64(&mut self, field_id: FieldId, value: i64) {
        unsafe { Schema_AddSint64(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn add_uint32(&mut self, field_id: FieldId, value: u32) {
        unsafe { Schema_AddUint32(&mut *self.inner as *mut Schema_Object, field_id, value) }
    }

    pub fn add_uint64(&mut self, field_id: FieldId, value: u64) {
        unsafe { Schema_AddUint64(&mut *self.inner as *mut Schema_Object, field_id, value) }
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

    pub fn add_bytes(&mut self, field_id: FieldId, values: &[u8]) {
        let buffer = self.allocate_buffer(values);
        unsafe {
            Schema_AddBytes(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                buffer.inner,
                values.len() as u32,
            )
        }
    }

    pub fn add_string<S: AsRef<str>>(&mut self, field_id: FieldId, value: S) {
        self.add_bytes(field_id, value.as_ref().as_bytes())
    }

    pub fn add_enum<E, V>(&mut self, field_id: FieldId, value: E)
    where
        for<'a> &'a V: Into<u32>,
        E: AsRef<V>,
    {
        unsafe {
            Schema_AddEnum(
                &mut *self.inner as *mut Schema_Object,
                field_id,
                value.as_ref().into(),
            )
        }
    }
}
