#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub(crate) mod ffi;
pub use ffi::*;

pub(crate) use ffi::{
    Worker_AddComponentOp, Worker_AddEntityOp, Worker_Authority, Worker_AuthorityChangeOp,
    Worker_CommandRequestOp, Worker_CommandResponseOp, Worker_ComponentId,
    Worker_ComponentUpdateOp, Worker_CreateEntityResponseOp, Worker_CriticalSectionOp,
    Worker_DeleteEntityResponseOp, Worker_DisconnectOp, Worker_EntityId,
    Worker_EntityQueryResponseOp, Worker_FlagUpdateOp, Worker_GaugeMetric, Worker_HistogramMetric,
    Worker_HistogramMetricBucket, Worker_LogLevel, Worker_LogMessageOp, Worker_Metrics, Worker_Op,
    Worker_OpType, Worker_WorkerAttributes, Worker_MetricsOp, Worker_RemoveComponentOp, Worker_RemoveEntityOp, Worker_ReserveEntityIdsResponseOp
};

#[cfg(target_os = "windows")]
extern crate gdi32;

#[cfg(target_os = "windows")]
extern crate user32;

pub mod worker;

/*
pub fn generate_worker(worker_type: String, worker_name: String) -> *mut Worker_Connection {
    let mut parameters = unsafe { Worker_DefaultConnectionParameters() };
    let worker_name = CString::new(worker_name).unwrap();
    let worker_type = CString::new(worker_type).unwrap();
    parameters.network.connection_type =
        Worker_NetworkConnectionType::WORKER_NETWORK_CONNECTION_TYPE_MODULAR_KCP as u8;
    parameters.network.modular_kcp.security_type =
        Worker_NetworkSecurityType::WORKER_NETWORK_SECURITY_TYPE_INSECURE as u8;
    parameters.worker_type = worker_type.as_ptr() as *const c_char;
    parameters.network.tcp.multiplex_level = 4;

    let hostname = CString::new("localhost").unwrap();

    unsafe {
        let connection_future = Worker_ConnectAsync(
            hostname.as_ptr() as *const c_char,
            7777,
            worker_name.as_ptr() as *const c_char,
            &parameters,
        );
        let connection = Worker_ConnectionFuture_Get(connection_future, null() as *const u32);
        Worker_ConnectionFuture_Destroy(connection_future);
        connection
    }
}
*/

/*
pub fn on_disconnect(op: DisconnectOp) {
    println!("Disconnected: {:?}", op.reason)
}

pub fn on_log_message(op: LogMessageOp) {
    println!("LogMessage: {:?}", op.message)
}

pub fn on_entity_query_response(op: Worker_EntityQueryResponseOp) {
    println!("EntityQueryResponse: {:?}", op)
}

pub fn on_add_entity(op: Worker_AddEntityOp) {
    println!("AddEntity: {:?}", op)
}

pub fn on_remove_entity(op: Worker_RemoveEntityOp) {
    println!("RemoveEntity: {:?}", op)
}

pub fn on_add_component(op: Worker_AddComponentOp) {
    println!("AddComponent: {:?}", op)
}

pub fn on_component_update(op: Worker_ComponentUpdateOp) {
    println!("ComponentUpdate: {:?}", op)
}

pub fn on_command_request(op: Worker_CommandRequestOp) {
    println!("CommandRequest: {:?}", op)
}

pub fn loop_worker(connection: *mut Worker_Connection) {
    let message = log_message::LogMessage::new(
        Worker_LogLevel::WORKER_LOG_LEVEL_WARN,
        "test_rust",
        "Connected successfully",
        None,
    );
    unsafe {
        Worker_Connection_SendLogMessage(connection, &message.0);
    };

    loop {
        let op_list = unsafe { Worker_Connection_GetOpList(connection, 0) };
        let op_count = unsafe { (*op_list).op_count };
        for op_idx in 0..op_count {
            let op = unsafe { *(*op_list).ops.offset(op_idx as isize) };
            match WorkerOp::from(op) {
                WorkerOp::Disconnect(disconnect) => on_disconnect(disconnect),
                WorkerOp::LogMessage(log_message) => on_log_message(log_message),
                WorkerOp::EntityQueryResponse(query_response) => {
                    on_entity_query_response(query_response)
                }
                WorkerOp::AddEntity(add_entity) => on_add_entity(add_entity),
                WorkerOp::RemoveEntity(remove_entity) => on_remove_entity(remove_entity),
                WorkerOp::AddComponent(add_component) => on_add_component(add_component),
                WorkerOp::ComponentUpdate(component_update) => {
                    on_component_update(component_update)
                }
                WorkerOp::CommandRequest(command_request) => on_command_request(command_request),
                _ => {}
            };
        }
        unsafe { Worker_OpList_Destroy(op_list) };
    }
}
*/
