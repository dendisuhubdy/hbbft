//! Networking controls of the consensus node.
use std::sync::mpsc;
use std::fmt::Debug;
use std::collections::{HashMap, HashSet};
use std::marker::{Send, Sync};
use std::net::{TcpStream, TcpListener, SocketAddr};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use broadcast::*;
use broadcast::stage::Stage as BroadcastStage;
use proto::Message;

/// This is a structure to start a consensus node.
pub struct Node {
    /// Incoming connection socket.
    addr: SocketAddr,
    /// Connection sockets of remote nodes. TODO.
    remotes: Vec<SocketAddr>
}

impl Node {
    pub fn new(addr: SocketAddr, remotes: Vec<SocketAddr>) -> Self {
        Node {addr, remotes}
    }

    pub fn run<T: Clone + Debug + Send + Sync + 'static>(&self) {
        // Listen for incoming connections on a given TCP port.
        let listener = TcpListener::bind(&self.addr).unwrap();
        let broadcast_stage: Arc<Mutex<BroadcastStage<T>>> =
            Arc::new(Mutex::new(BroadcastStage::new(Vec::new())));

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    info!("New connection from {:?}",
                          stream.peer_addr().unwrap());
                    let (tx, rx): (Sender<Message<T>>, Receiver<Message<T>>) =
                        channel();
                    let stage = Arc::clone(&broadcast_stage);
                    // Insert the transmit handle connected to this task into
                    // the shared list of senders.
                    stage.lock().unwrap().senders.push(tx);
                    let task = BroadcastTask::new(stream, rx, stage);

                    // TODO: spawn a thread for the connected socket
                }
                Err(e) => {
                    warn!("Failed to connect: {}", e);
                }
            }
        }
    }
}
