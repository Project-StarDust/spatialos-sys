use super::Object;
use crate::ffi::*;
use crate::schema::FieldId;

impl Object {
    pub fn get_bool_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetBoolCount(&*self.inner as *const Schema_Object, field_id) }
    }
    pub fn get_double_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetDoubleCount(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_uint32_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetUint32Count(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_object_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetObjectCount(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_bytes_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetBytesCount(&*self.inner as *const Schema_Object, field_id) }
    }

    pub fn get_enum_count(&self, field_id: FieldId) -> u32 {
        unsafe { Schema_GetEnumCount(&*self.inner as *const Schema_Object, field_id) }
    }
}
