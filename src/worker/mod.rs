pub mod log_message;
pub mod op;
pub mod metrics;

use crate::ffi::*;
use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::c_char;

pub type EntityId = i64;
pub type ComponentId = u32;
pub type RequestId = i64;

#[doc = " Enum defining the severities of log messages that can be sent to SpatialOS and received from the"]
#[doc = " SDK."]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl From<Worker_LogLevel> for LogLevel {
    fn from(log_level: Worker_LogLevel) -> Self {
        match log_level {
            Worker_LogLevel::WORKER_LOG_LEVEL_DEBUG => Self::Debug,
            Worker_LogLevel::WORKER_LOG_LEVEL_INFO => Self::Info,
            Worker_LogLevel::WORKER_LOG_LEVEL_WARN => Self::Warn,
            Worker_LogLevel::WORKER_LOG_LEVEL_ERROR => Self::Error,
            Worker_LogLevel::WORKER_LOG_LEVEL_FATAL => Self::Fatal,
        }
    }
}

impl From<u8> for Worker_LogLevel {
    fn from(log_level: u8) -> Self {
        match log_level {
            1 => Worker_LogLevel::WORKER_LOG_LEVEL_DEBUG,
            2 => Worker_LogLevel::WORKER_LOG_LEVEL_INFO,
            3 => Worker_LogLevel::WORKER_LOG_LEVEL_WARN,
            4 => Worker_LogLevel::WORKER_LOG_LEVEL_ERROR,
            5 => Worker_LogLevel::WORKER_LOG_LEVEL_FATAL,
            _ => panic!("Invalid byte"),
        }
    }
}

impl From<u8> for LogLevel {
    fn from(log_level: u8) -> Self {
        LogLevel::from(Worker_LogLevel::from(log_level))
    }
}

#[doc = " Enum defining the possible authority states for an entity component."]
pub enum Authority {
    NotAuthoritative,
    Authoritative,
    AuthorityLossImminent,
}

impl From<Worker_Authority> for Authority {
    fn from(authority: Worker_Authority) -> Self {
        match authority {
            Worker_Authority::WORKER_AUTHORITY_NOT_AUTHORITATIVE => Self::NotAuthoritative,
            Worker_Authority::WORKER_AUTHORITY_AUTHORITATIVE => Self::Authoritative,
            Worker_Authority::WORKER_AUTHORITY_AUTHORITY_LOSS_IMMINENT => {
                Self::AuthorityLossImminent
            }
        }
    }
}

impl From<u8> for Worker_Authority {
    fn from(authority: u8) -> Self {
        match authority {
            0 => Worker_Authority::WORKER_AUTHORITY_NOT_AUTHORITATIVE,
            1 => Worker_Authority::WORKER_AUTHORITY_AUTHORITATIVE,
            2 => Worker_Authority::WORKER_AUTHORITY_AUTHORITY_LOSS_IMMINENT,
            _ => panic!("Invalid byte"),
        }
    }
}

impl From<u8> for Authority {
    fn from(authority: u8) -> Self {
        Authority::from(Worker_Authority::from(authority))
    }
}

pub enum Worker_StatusCode {
    WORKER_STATUS_CODE_SUCCESS,
    WORKER_STATUS_CODE_TIMEOUT,
    WORKER_STATUS_CODE_NOT_FOUND,
    WORKER_STATUS_CODE_AUTHORITY_LOST,
    WORKER_STATUS_CODE_PERMISSION_DENIED,
    WORKER_STATUS_CODE_APPLICATION_ERROR,
    WORKER_STATUS_CODE_INTERNAL_ERROR,
}

#[doc = " Enum defining possible command status codes."]
pub enum StatusCode {
    #[doc = " The request was successfully executed and returned a response."]
    Success,
    #[doc = " The request timed out before a response was received. It can be retried, but carefully - this"]
    #[doc = " usually means the deployment is overloaded, so some sort of backoff should be used to avoid"]
    #[doc = " making the problem worse. This can also be caused by the target worker's handling code failing"]
    #[doc = " to respond to the command at all, perhaps due to a bug in its implementation."]
    Timeout,
    #[doc = " The target entity did not exist, or did not have the target component. This probably means the"]
    #[doc = " entity either hasn't been created yet or has already been deleted. It might make sense to retry"]
    #[doc = " the request if there is reason to believe the entity hasn't yet been created but will be soon."]
    NotFound,
    #[doc = " The request could not be executed by a worker, either because the worker lost authority over"]
    #[doc = " the entity while handling the request, the entity was deleted while handling the request, or no"]
    #[doc = " worker was authoritative over the entity at all. Assuming the deployment isn't irrecoverably"]
    #[doc = " broken (e.g. due to misconfigured loadbalancing or crash-looping workers) this is a transient"]
    #[doc = " failure and can be retried immediately."]
    AuthorityLost,
    #[doc = " The worker did not have the required permissions to make the request. Permissions do not change"]
    #[doc = " at runtime, so it doesn't make sense to retry the request."]
    PermissionDenied,
    #[doc = " The command was delivered successfully, but the handler rejected it. Either the command was"]
    #[doc = " delivered to a worker that explicitly rejected it by calling"]
    #[doc = " Worker_Connection_SendCommandFailure, or the request data was rejected as invalid by SpatialOS"]
    #[doc = " itself. In the latter case, in particular, Worker_Connection_SendCreateEntityRequest will"]
    #[doc = " return kApplicationError if an entity ID reservation has expired, and"]
    #[doc = " Worker_Connection_SendEntityQueryResult will return kApplicationError if the result set is"]
    #[doc = " incomplete."]
    ApplicationError,
    #[doc = " Some other error occurred. This likely indicates a bug in SpatialOS and should be reported."]
    InternalError,
}

impl From<Worker_StatusCode> for StatusCode {
    fn from(status_code: Worker_StatusCode) -> Self {
        match status_code {
            Worker_StatusCode::WORKER_STATUS_CODE_SUCCESS => Self::Success,
            Worker_StatusCode::WORKER_STATUS_CODE_TIMEOUT => Self::Timeout,
            Worker_StatusCode::WORKER_STATUS_CODE_NOT_FOUND => Self::NotFound,
            Worker_StatusCode::WORKER_STATUS_CODE_AUTHORITY_LOST => Self::AuthorityLost,
            Worker_StatusCode::WORKER_STATUS_CODE_PERMISSION_DENIED => Self::PermissionDenied,
            Worker_StatusCode::WORKER_STATUS_CODE_APPLICATION_ERROR  => Self::ApplicationError,
            Worker_StatusCode::WORKER_STATUS_CODE_INTERNAL_ERROR  => Self::InternalError,
        }
    }
}

impl From<u8> for Worker_StatusCode {
    fn from(status_code: u8) -> Self {
        match status_code {
            1 => Worker_StatusCode::WORKER_STATUS_CODE_SUCCESS,
            2 => Worker_StatusCode::WORKER_STATUS_CODE_TIMEOUT,
            3 => Worker_StatusCode::WORKER_STATUS_CODE_NOT_FOUND,
            4 => Worker_StatusCode::WORKER_STATUS_CODE_AUTHORITY_LOST,
            5 => Worker_StatusCode::WORKER_STATUS_CODE_PERMISSION_DENIED,
            6 => Worker_StatusCode::WORKER_STATUS_CODE_APPLICATION_ERROR,
            7 => Worker_StatusCode::WORKER_STATUS_CODE_INTERNAL_ERROR,
            _ => panic!("Invalid byte"),
        }
    }
}

impl From<u8> for StatusCode {
    fn from(status_code: u8) -> Self {
        StatusCode::from(Worker_StatusCode::from(status_code))
    }
}

#[doc = " Worker attributes that are part of a worker's runtime configuration."]
pub struct WorkerAttributes {
    #[doc = " Number of worker attributes."]
    pub attribute_count: u32,
    #[doc = " Will be empty if there are no attributes associated with the worker."]
    pub attributes: Vec<String>,
}

impl From<Worker_WorkerAttributes> for WorkerAttributes {
    fn from(worker_attributes: Worker_WorkerAttributes) -> Self {
        if worker_attributes.attributes.is_null() {
            Self {
                attribute_count: worker_attributes.attribute_count,
                attributes: Vec::new(),
            }
        } else {
            let attributes = unsafe {
                let index = 0;
                let mut attributes = Vec::new();
                loop {
                    let char_ptr = worker_attributes
                        .attributes
                        .offset(index * size_of::<*const c_char>() as isize);
                    if (*char_ptr).is_null() {
                        break;
                    } else {
                        let attribute = CStr::from_ptr(*char_ptr)
                            .to_str()
                            .map(|s| s.to_owned())
                            .unwrap();
                        attributes.push(attribute);
                    }
                }
                attributes
            };
            Self {
                attribute_count: worker_attributes.attribute_count,
                attributes,
            }
        }
    }
}
