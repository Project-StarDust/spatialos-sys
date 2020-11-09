#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub(crate) mod ffi;
pub use ffi::*;

#[allow(unused_imports)]
pub(crate) use ffi::{
    Worker_AddComponentOp, Worker_AddEntityOp, Worker_Authority, Worker_AuthorityChangeOp,
    Worker_CommandRequestOp, Worker_CommandResponseOp, Worker_ComponentId,
    Worker_ComponentUpdateOp, Worker_ConnectionFuture, Worker_ConnectionFuture_Get,
    Worker_ConnectionParameters, Worker_ConnectionStatusCode, Worker_Connection_Destroy,
    Worker_Connection_GetOpList, Worker_Connection_SendLogMessage, Worker_CreateEntityResponseOp,
    Worker_CriticalSectionOp, Worker_DefaultConnectionParameters, Worker_DeleteEntityResponseOp,
    Worker_DisconnectOp, Worker_EntityId, Worker_EntityQueryResponseOp, Worker_FlagUpdateOp,
    Worker_GaugeMetric, Worker_HistogramMetric, Worker_HistogramMetricBucket, Worker_LogLevel,
    Worker_LogMessage, Worker_LogMessageOp, Worker_Metrics, Worker_MetricsOp,
    Worker_ModularKcpNetworkParameters, Worker_NetworkConnectionType, Worker_NetworkParameters,
    Worker_NetworkSecurityType, Worker_Op, Worker_OpList_Destroy, Worker_OpType, Worker_Op_Union,
    Worker_RemoveComponentOp, Worker_RemoveEntityOp, Worker_RequestId,
    Worker_ReserveEntityIdsResponseOp, Worker_WorkerAttributes,
};

#[cfg(target_os = "windows")]
extern crate gdi32;

#[cfg(target_os = "windows")]
extern crate user32;

pub mod schema;
pub mod worker;
