#![allow(dead_code)]
extern crate mio;

use self::mio::Token;
use self::mio::channel::{Sender};
use network::tcp::{TcpWriterCommand, TcpWriterCMD, TcpReaderConn};
use std::sync::Arc;

pub enum ConnectionType {
    TCP
}

/// Base Connection structure for handling base information of connection
pub struct Connection {
    pub value: u64,
    pub api_version: u32,
    pub conn_type: ConnectionType,
    pub from_server: bool,

    // writer command for TCP connection or None if this is not a TCP connection
    tcp_writer_chan: Vec<Sender<TcpWriterCommand>>,
    tcp_writer_token: Vec<Token>
}

impl Connection {
    #[inline(always)]
    pub fn from_tcp(tcp_conn: &TcpReaderConn, writer: Sender<TcpWriterCommand>, from_server: bool) -> Connection {
        Connection {
            value: tcp_conn.value,
            api_version: tcp_conn.api_version,
            conn_type: ConnectionType::TCP,
            from_server: from_server,
            tcp_writer_chan: vec![writer],
            tcp_writer_token: vec![tcp_conn.socket_token]
        }
    }

    #[inline(always)]
    pub fn write_tcp(&self, data: Arc<Vec<u8>>) {
        if self.tcp_writer_chan.len() == 0 {
            return;
        }

        let _ = self.tcp_writer_chan[0].send(TcpWriterCommand {
            cmd: TcpWriterCMD::WriteData,
            conn: vec![],
            data: vec![data],
            token: self.tcp_writer_token.clone(),
        });
    }
}