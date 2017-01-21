use std::sync::{Arc, Mutex};

use opcua_core::types::*;
use opcua_core::comms::*;

use subscription::{Subscription};
use services::discovery::*;
use services::session::*;
use server::ServerState;
use tcp_session::SessionState;

/// Processes and dispatches messages for handling
pub struct MessageHandler {
    /// Server state
    server_state: Arc<Mutex<ServerState>>,
    /// Session state
    session_state: Arc<Mutex<SessionState>>,
    /// Discovery service
    discovery_service: DiscoveryService,
    /// Session service
    session_service: SessionService,
}

impl MessageHandler {
    pub fn new(server_state: &Arc<Mutex<ServerState>>, session_state: &Arc<Mutex<SessionState>>) -> MessageHandler {
        MessageHandler {
            server_state: server_state.clone(),
            session_state: session_state.clone(),
            discovery_service: DiscoveryService::new(),
            session_service: SessionService::new(),
        }
    }

    pub fn handle_message(&self, message: &SupportedMessage) -> Result<SupportedMessage, &'static StatusCode> {
        let mut server_state = self.server_state.lock().unwrap();
        let mut session_state = self.session_state.lock().unwrap();
        let response = match *message {
            SupportedMessage::GetEndpointsRequest(ref request) => {
                self.discovery_service.handle_get_endpoints_request(&mut server_state, &mut session_state, request)?
            },
            SupportedMessage::CreateSessionRequest(ref request) => {
                self.session_service.handle_create_session_request(&mut server_state, &mut session_state, request)?
            },
            SupportedMessage::CloseSessionRequest(ref request) => {
                self.session_service.handle_close_session_request(&mut server_state, &mut session_state, request)?
            }
            _ => {
                debug!("Message handler does not handle this kind of message");
                return Err(&BAD_SERVICE_UNSUPPORTED);
            }
        };
        Ok(response)
    }
}
