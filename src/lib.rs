cargo_component_bindings::generate!();

use bindings::component::uq_process::types::*;
use bindings::{
    get_payload, print_to_terminal, receive, send_and_await_response, send_request, send_requests,
    send_response, Guest,
};

mod process_lib;

struct Component;

impl Guest for Component {
    fn init(our: Address) {
        print_to_terminal(0, "__NAME__: start");

        loop {
            let Ok((source, message)) = receive() else {
                print_to_terminal(0, "__NAME__: got network error");
                continue;
            };
            let Message::Request(request) = message else {
                print_to_terminal(0, "__NAME__: got unexpected message");
                continue;
            };

            if let Some(json) = request.ipc {
                print_to_terminal(0, format!("__NAME__: JSON {}", json).as_str());
                let message_json: serde_json::Value = match serde_json::from_str(&json) {
                    Ok(v) => v,
                    Err(_) => {
                        print_to_terminal(1, "__NAME__: failed to parse ipc JSON, skipping");
                        continue;
                    }
                };

                print_to_terminal(0, &format!("__NAME__: parsed message: {}", json));
            }
        }
    }
}
