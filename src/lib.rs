#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(target_os = "windows")]
extern crate gdi32;

#[cfg(target_os = "windows")]
extern crate user32;

impl From<u8> for Worker_NetworkConnectionType {
    fn from(connection_type: u8) -> Self {
        match connection_type {
            0 => Self::WORKER_NETWORK_CONNECTION_TYPE_TCP,
            1 => Self::WORKER_NETWORK_CONNECTION_TYPE_RAKNET,
            2 => Self::WORKER_NETWORK_CONNECTION_TYPE_KCP,
            3 => Self::WORKER_NETWORK_CONNECTION_TYPE_MODULAR_KCP,
            4 => Self::WORKER_NETWORK_CONNECTION_TYPE_MODULAR_TCP,
            _ => panic!("Invalid byte"),
        }
    }
}

impl Into<u8> for Worker_NetworkConnectionType {
    fn into(self) -> u8 {
        match self {
            Self::WORKER_NETWORK_CONNECTION_TYPE_TCP => 0,
            Self::WORKER_NETWORK_CONNECTION_TYPE_RAKNET => 1,
            Self::WORKER_NETWORK_CONNECTION_TYPE_KCP => 2,
            Self::WORKER_NETWORK_CONNECTION_TYPE_MODULAR_KCP => 3,
            Self::WORKER_NETWORK_CONNECTION_TYPE_MODULAR_TCP => 4,
        }
    }
}


impl From<u8> for Worker_NetworkSecurityType {
    fn from(security_type: u8) -> Self {
        match security_type {
            0 => Self::WORKER_NETWORK_SECURITY_TYPE_INSECURE,
            1 => Self::WORKER_NETWORK_SECURITY_TYPE_TLS,
            _ => panic!("Invalid byte"),
        }
    }
}


impl Into<u8> for Worker_NetworkSecurityType {
    fn into(self) -> u8 {
        match self {
            Self::WORKER_NETWORK_SECURITY_TYPE_INSECURE => 0,
            Self::WORKER_NETWORK_SECURITY_TYPE_TLS => 1,
        }
    }
}


impl From<u8> for Worker_ConstraintType {
    fn from(data: u8) -> Self {
        match data {
            1 => Self::WORKER_CONSTRAINT_TYPE_ENTITY_ID,
            2 => Self::WORKER_CONSTRAINT_TYPE_COMPONENT,
            3 => Self::WORKER_CONSTRAINT_TYPE_SPHERE,
            4 => Self::WORKER_CONSTRAINT_TYPE_AND,
            5 => Self::WORKER_CONSTRAINT_TYPE_OR,
            6 => Self::WORKER_CONSTRAINT_TYPE_NOT,
            _ => panic!("Invalid byte: {}", data),
        }
    }
}

impl Into<u8> for Worker_ConstraintType {
    fn into(self) -> u8 {
        match self {
            Self::WORKER_CONSTRAINT_TYPE_ENTITY_ID => 1,
            Self::WORKER_CONSTRAINT_TYPE_COMPONENT => 2,
            Self::WORKER_CONSTRAINT_TYPE_SPHERE => 3,
            Self::WORKER_CONSTRAINT_TYPE_AND => 4,
            Self::WORKER_CONSTRAINT_TYPE_OR => 5,
            Self::WORKER_CONSTRAINT_TYPE_NOT => 6,
        }
    }
}

impl From<u8> for Worker_LogLevel {
    fn from(log_level: u8) -> Self {
        match log_level {
            1 => Self::WORKER_LOG_LEVEL_DEBUG,
            2 => Self::WORKER_LOG_LEVEL_INFO,
            3 => Self::WORKER_LOG_LEVEL_WARN,
            4 => Self::WORKER_LOG_LEVEL_ERROR,
            5 => Self::WORKER_LOG_LEVEL_FATAL,
            _ => panic!("Invalid byte"),
        }
    }
}

impl Into<u8> for Worker_LogLevel {
    fn into(self) -> u8 {
        match self {
            Self::WORKER_LOG_LEVEL_DEBUG => 1,
            Self::WORKER_LOG_LEVEL_INFO => 2,
            Self::WORKER_LOG_LEVEL_WARN => 3,
            Self::WORKER_LOG_LEVEL_ERROR => 4,
            Self::WORKER_LOG_LEVEL_FATAL => 5,
        }
    }
}

impl From<u8> for Worker_Authority {
    fn from(authority: u8) -> Self {
        match authority {
            0 => Self::WORKER_AUTHORITY_NOT_AUTHORITATIVE,
            1 => Self::WORKER_AUTHORITY_AUTHORITATIVE,
            2 => Self::WORKER_AUTHORITY_AUTHORITY_LOSS_IMMINENT,
            _ => panic!("Invalid byte"),
        }
    }
}

impl From<u8> for Worker_StatusCode {
    fn from(status_code: u8) -> Self {
        match status_code {
            1 => Self::WORKER_STATUS_CODE_SUCCESS,
            2 => Self::WORKER_STATUS_CODE_TIMEOUT,
            3 => Self::WORKER_STATUS_CODE_NOT_FOUND,
            4 => Self::WORKER_STATUS_CODE_AUTHORITY_LOST,
            5 => Self::WORKER_STATUS_CODE_PERMISSION_DENIED,
            6 => Self::WORKER_STATUS_CODE_APPLICATION_ERROR,
            7 => Self::WORKER_STATUS_CODE_INTERNAL_ERROR,
            _ => panic!("Invalid byte"),
        }
    }
}

impl From<u8> for Worker_ConnectionStatusCode {
    fn from(status_code: u8) -> Self {
        match status_code {
            1 => Self::WORKER_CONNECTION_STATUS_CODE_SUCCESS,
            2 => Self::WORKER_CONNECTION_STATUS_CODE_INTERNAL_ERROR,
            3 => Self::WORKER_CONNECTION_STATUS_CODE_INVALID_ARGUMENT,
            4 => Self::WORKER_CONNECTION_STATUS_CODE_NETWORK_ERROR,
            5 => Self::WORKER_CONNECTION_STATUS_CODE_TIMEOUT,
            6 => Self::WORKER_CONNECTION_STATUS_CODE_CANCELLED,
            7 => Self::WORKER_CONNECTION_STATUS_CODE_REJECTED,
            8 => Self::WORKER_CONNECTION_STATUS_CODE_PLAYER_IDENTITY_TOKEN_EXPIRED,
            9 => Self::WORKER_CONNECTION_STATUS_CODE_LOGIN_TOKEN_EXPIRED,
            10 => Self::WORKER_CONNECTION_STATUS_CODE_CAPACITY_EXCEEDED,
            11 => Self::WORKER_CONNECTION_STATUS_CODE_RATE_EXCEEDED,
            12 => Self::WORKER_CONNECTION_STATUS_CODE_SERVER_SHUTDOWN,
            _ => panic!("Invalid byte"),
        }
    }
}

impl From<u8> for Worker_ResultType {
    fn from(result_type: u8) -> Self {
        match result_type {
            1 => Self::WORKER_RESULT_TYPE_COUNT,
            2 => Self::WORKER_RESULT_TYPE_SNAPSHOT,
            _ => panic!("Invalid byte"),
        }
    }
}


impl Into<u8> for Worker_ResultType {
    fn into(self) -> u8 {
        match self {
            Self::WORKER_RESULT_TYPE_COUNT => 1,
            Self::WORKER_RESULT_TYPE_SNAPSHOT => 2,
        }
    }
}

impl From<u8> for Worker_OpType {
    fn from(data: u8) -> Self {
        match data {
            1 => Self::WORKER_OP_TYPE_DISCONNECT,
            2 => Self::WORKER_OP_TYPE_FLAG_UPDATE,
            3 => Self::WORKER_OP_TYPE_LOG_MESSAGE,
            4 => Self::WORKER_OP_TYPE_METRICS,
            5 => Self::WORKER_OP_TYPE_CRITICAL_SECTION,
            6 => Self::WORKER_OP_TYPE_ADD_ENTITY,
            7 => Self::WORKER_OP_TYPE_REMOVE_ENTITY,
            8 => Self::WORKER_OP_TYPE_RESERVE_ENTITY_IDS_RESPONSE,
            9 => Self::WORKER_OP_TYPE_CREATE_ENTITY_RESPONSE,
            10 => Self::WORKER_OP_TYPE_DELETE_ENTITY_RESPONSE,
            11 => Self::WORKER_OP_TYPE_ENTITY_QUERY_RESPONSE,
            12 => Self::WORKER_OP_TYPE_ADD_COMPONENT,
            13 => Self::WORKER_OP_TYPE_REMOVE_COMPONENT,
            14 => Self::WORKER_OP_TYPE_AUTHORITY_CHANGE,
            15 => Self::WORKER_OP_TYPE_COMPONENT_UPDATE,
            16 => Self::WORKER_OP_TYPE_COMMAND_REQUEST,
            17 => Self::WORKER_OP_TYPE_COMMAND_RESPONSE,
            _ => panic!("Invalid byte: {}", data),
        }
    }
}
