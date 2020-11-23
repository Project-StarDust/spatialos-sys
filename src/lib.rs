#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub(crate) mod ffi;
pub use ffi::*;

#[allow(unused_imports)]
pub(crate) use ffi::{
    Worker_AddComponentOp, Worker_AddEntityOp, Worker_AndConstraint, Worker_Authority,
    Worker_AuthorityChangeOp, Worker_CommandIndex, Worker_CommandRequestCopy,
    Worker_CommandRequestDeserialize, Worker_CommandRequestFree, Worker_CommandRequestOp,
    Worker_CommandRequestSerialize, Worker_CommandResponseCopy, Worker_CommandResponseDeserialize,
    Worker_CommandResponseFree, Worker_CommandResponseHandle, Worker_CommandResponseOp,
    Worker_CommandResponseSerialize, Worker_ComponentConstraint, Worker_ComponentData,
    Worker_ComponentDataCopy, Worker_ComponentDataDeserialize, Worker_ComponentDataFree,
    Worker_ComponentDataHandle, Worker_ComponentDataSerialize, Worker_ComponentId,
    Worker_ComponentUpdateCopy, Worker_ComponentUpdateDeserialize, Worker_ComponentUpdateFree,
    Worker_ComponentUpdateHandle, Worker_ComponentUpdateOp, Worker_ComponentUpdateSerialize,
    Worker_ConnectAsync, Worker_Connection, Worker_ConnectionFuture,
    Worker_ConnectionFuture_Destroy, Worker_ConnectionFuture_Get, Worker_ConnectionParameters,
    Worker_ConnectionStatusCode, Worker_Connection_Destroy, Worker_Connection_GetOpList,
    Worker_Connection_SendEntityQueryRequest, Worker_Connection_SendLogMessage, Worker_Constraint,
    Worker_ConstraintType, Worker_Constraint_Union, Worker_CreateEntityResponseOp,
    Worker_CriticalSectionOp, Worker_DefaultConnectionParameters, Worker_DeleteEntityResponseOp,
    Worker_DisconnectOp, Worker_Entity, Worker_EntityId, Worker_EntityIdConstraint,
    Worker_EntityQuery, Worker_EntityQueryResponseOp, Worker_FlagUpdateOp, Worker_GaugeMetric,
    Worker_HistogramMetric, Worker_HistogramMetricBucket, Worker_LogLevel, Worker_LogMessage,
    Worker_LogMessageOp, Worker_Metrics, Worker_MetricsOp, Worker_ModularKcpNetworkParameters,
    Worker_NetworkConnectionType, Worker_NetworkParameters, Worker_NetworkSecurityType,
    Worker_NotConstraint, Worker_Op, Worker_OpList_Destroy, Worker_OpType, Worker_Op_Union,
    Worker_OrConstraint, Worker_RemoveComponentOp, Worker_RemoveEntityOp, Worker_RequestId,
    Worker_ReserveEntityIdsResponseOp, Worker_ResultType, Worker_SphereConstraint,
    Worker_StatusCode, Worker_WorkerAttributes,
};

#[allow(unused_imports)]
pub(crate) use ffi::{
    Schema_AddBool, Schema_AddBytes, Schema_AddDouble, Schema_AddDoubleList, Schema_AddEntityId,
    Schema_AddFixed32, Schema_AddFixed64, Schema_AddFloat, Schema_AddInt32, Schema_AddInt64,
    Schema_AddObject, Schema_AddSfixed32, Schema_AddSfixed64, Schema_AddSint32, Schema_AddSint64,
    Schema_AddUint32, Schema_AddUint64, Schema_AllocateBuffer, Schema_CreateComponentData,
    Schema_CreateComponentUpdate, Schema_GetBool, Schema_GetBoolCount, Schema_GetBytes,
    Schema_GetBytesCount, Schema_GetBytesLength, Schema_GetComponentDataFields,
    Schema_GetComponentUpdateFields, Schema_GetDouble, Schema_GetDoubleCount, Schema_GetDoubleList,
    Schema_GetEntityId, Schema_GetFixed32, Schema_GetFixed64, Schema_GetFloat, Schema_GetInt32,
    Schema_GetInt64, Schema_GetObject, Schema_GetObjectCount, Schema_GetSfixed32,
    Schema_GetSfixed64, Schema_GetSint32, Schema_GetSint64, Schema_GetUint32,
    Schema_GetUint32Count, Schema_GetUint32List, Schema_GetUint64, Schema_IndexBytes,
    Schema_IndexBytesLength, Schema_IndexObject, SCHEMA_MAP_KEY_FIELD_ID,
    SCHEMA_MAP_VALUE_FIELD_ID, Schema_AddUint32List
};

#[cfg(target_os = "windows")]
extern crate gdi32;

#[cfg(target_os = "windows")]
extern crate user32;

pub mod schema;
pub mod worker;

pub(crate) fn mut_to_vector<T>(data: *mut T, size: isize) -> Vec<T> {
    if data.is_null() {
        Vec::new()
    } else {
        let mut datas = Vec::new();
        for index in 0..size {
            let data = unsafe {
                let data_ptr = data.offset(index);
                Box::from_raw(data_ptr)
            };
            datas.push(*data);
        }
        datas
    }
}

pub(crate) fn const_to_vector<T: Clone>(data: *const T, size: isize) -> Vec<T> {
    if data.is_null() {
        Vec::new()
    } else {
        let mut datas = Vec::new();
        for index in 0..size {
            let data = unsafe {
                let data_ptr = data.offset(index);
                (*data_ptr).clone()
            };
            datas.push(data);
        }
        datas
    }
}
