use spatialos_sys::worker::{connection_parameters::NetworkSecurityType, op::{AddComponentOp, AddEntityOp, CommandRequestOp, ComponentUpdateOp, DisconnectOp, EntityQueryResponseOp, LogMessageOp, RemoveEntityOp, WorkerOp}};
use spatialos_sys::worker::connection_parameters::{ConnectionFuture, ConnectionParameters};
use spatialos_sys::worker::{
    connection_parameters::NetworkConnectionType, log_message::LogMessage, LogLevel,
};

pub fn on_disconnect(op: &DisconnectOp) {
    println!("Disconnected: {:?}", op)
}

pub fn on_log_message(op: &LogMessageOp) {
    println!("LogMessage: {:?}", op)
}

pub fn on_entity_query_response(op: &EntityQueryResponseOp) {
    println!("EntityQueryResponse: {:?}", op)
}

pub fn on_add_entity(op: &AddEntityOp) {
    println!("AddEntity: {:?}", op)
}

pub fn on_remove_entity(op: &RemoveEntityOp) {
    println!("RemoveEntity: {:?}", op)
}

pub fn on_add_component(op: &AddComponentOp) {
    println!("AddComponent: {:?}", op)
}

pub fn on_component_update(op: &ComponentUpdateOp) {
    println!("ComponentUpdate: {:?}", op)
}

pub fn on_command_request(op: &CommandRequestOp) {
    println!("CommandRequest: {:?}", op)
}

fn main() {
    let parameters = {
        let mut parameters = ConnectionParameters::default();
        parameters.network.connection_type = NetworkConnectionType::ModularKcp;
        parameters.network.modular_kcp.security_type = NetworkSecurityType::Insecure;
        parameters.worker_type = "test_rust".to_string();
        parameters.network.tcp.multiplex_level = 4;
        parameters.default_component_vtable = std::ptr::null();
        parameters
    };
    let mut connection =
        ConnectionFuture::connect_async("localhost", 7777, "test_rust9", parameters);
    let mut new_connection = connection.get(None).expect("Timed out");
    new_connection.send_log_message(LogMessage::new(
        LogLevel::Warn,
        "test_rust",
        "Salut a tous !",
        None,
    ));

    loop {
        let op_list = new_connection.get_op_list(0);
        for op in op_list.ops.iter() {
            match op {
                WorkerOp::Disconnect(disconnect) => on_disconnect(&disconnect),
                WorkerOp::LogMessage(log_message) => on_log_message(&log_message),
                WorkerOp::EntityQueryResponse(query_response) => {
                    on_entity_query_response(&query_response)
                }
                WorkerOp::AddEntity(add_entity) => on_add_entity(&add_entity),
                WorkerOp::RemoveEntity(remove_entity) => on_remove_entity(&remove_entity),
                WorkerOp::AddComponent(add_component) => on_add_component(&add_component),
                WorkerOp::ComponentUpdate(component_update) => {
                    on_component_update(&component_update)
                }
                WorkerOp::CommandRequest(command_request) => on_command_request(&command_request),
                _ => {}
            };
        }
    }
}
