use std::ptr::null;
use std::ffi::CString;
use std::os::raw::c_char;

use crate::ffi::Worker_LogLevel;
use crate::ffi::Worker_EntityId;
use crate::ffi::Worker_LogMessage;

pub struct LogMessage(pub Worker_LogMessage);

impl LogMessage {
    pub fn new<S: AsRef<str>>(
        level: Worker_LogLevel,
        logger_name: S,
        message: S,
        entity_id: Option<Worker_EntityId>,
    ) -> Self {
        let logger_name = CString::new(logger_name.as_ref()).unwrap();
        let message = CString::new(message.as_ref()).unwrap();
        let log_message = Worker_LogMessage {
            level: level as u8,
            logger_name: logger_name.as_ptr() as *const c_char,
            message: message.as_ptr() as *const c_char,
            entity_id: match entity_id {
                Some(entity_id) => entity_id as *const Worker_EntityId,
                None => null() as *const Worker_EntityId,
            },
        };
        Self(log_message)
    }
}