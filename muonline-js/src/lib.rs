#[macro_use]
extern crate log;
extern crate tap;

extern crate futures;
extern crate tokio;

extern crate muonline_gs as mugs;
extern crate muonline_packet as mupack;
extern crate muonline_packet_codec as mucodec;
extern crate muonline_protocol as protocol;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate jsonrpc_macros;
extern crate jsonrpc_core;
extern crate jsonrpc_http_server;

#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

// TODO: Replace all unwraps with expect
// TODO: Figure out Error string formatting
// TODO: Determine how logging output should be

pub use builder::ServerBuilder;
use std::io;
use traits::QueryableGameServer;

#[macro_use]
mod macros;
mod builder;
mod controller;
mod service;
mod traits;
pub mod rpc {
  // Re-export the RPC API
  pub use service::rpc::api::*;
}

/// An implementation of a Join Server.
pub struct JoinServer {
  #[allow(unused)]
  game_servers: Vec<Box<QueryableGameServer>>,
  join_service: service::JoinService,
  rpc_service: service::RpcService,
}

impl JoinServer {
  /// Spawns a new Join Server using defaults.
  pub fn spawn() -> io::Result<Self> { Self::builder().spawn() }

  /// Returns a builder for the Join Server.
  pub fn builder() -> ServerBuilder { ServerBuilder::default() }

  /// Returns the URI of the RPC service.
  pub fn uri(&self) -> &str { self.rpc_service.uri() }

  /// Closes the server.
  pub fn close(self) -> io::Result<()> {
    let result = self.join_service.close();
    self.rpc_service.close();
    result
  }

  /// Will block, waiting for the server to finish.
  pub fn wait(self) -> io::Result<()> {
    let result = self.join_service.wait();
    self.rpc_service.close();
    result
  }
}
