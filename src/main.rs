#[allow(dead_code)]
#[allow(unused_imports)]
mod lamport_clock;
mod event;

use lamport_clock::{Clock, LamportClock};
use serde::{Deserialize, Serialize};
use clap::{Parser};
use event::Message;
use std::{sync::{Arc, Mutex}, io::Write};

#[derive(Clone)]
struct Node {
    handler: NodeHandler<Message>,
    port: String,
    clock: LamportClock,
}

use message_io::node::{self, NodeHandler, NodeListener, NodeEvent};
use message_io::network::{NetEvent, Transport};
use once_cell::sync::Lazy;

static MESSAGE_QUEUE: Lazy<Arc<Mutex<Vec<Message>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(Vec::new()))
});

impl Node {
    fn new(port: &str) -> Self {
        let (handler, listener) = node::split::<Message>();

        let node = Node {
            handler,
            port: port.to_owned(),
            clock: LamportClock::new(),
        };

        node.start_receiver(listener, port);

        node
    }

    fn send(&self, port: &str, msg: &str) {
        let (server, _) = self.handler.network().connect(Transport::Udp, format!("127.0.0.1:{port}")).unwrap();
        println!("sending on {port}");
        let message = Message{
            data: msg.to_owned(),
            sender_id: self.port.to_owned(),
            time_stamp: self.clock.get_current_timestam()
        };

        let data = serde_json::to_vec(&message).unwrap();
        Node::store_message(message);
        self.handler.network().send(server, &data);
    }

    fn start_receiver(&self, listener: NodeListener<Message>, port: &str) {
        tokio::spawn({
            let handler = self.handler.clone();
            let port = port.to_owned();
            async move {
                // Listen for TCP, UDP and WebSocket messages at the same time.
                handler.network().listen(Transport::Udp, format!("0.0.0.0:{port}")).unwrap();

                // Read incoming network events.
                listener.for_each(move |event| match event {
                    node::NodeEvent::Network(net_event) => match net_event{
                        NetEvent::Connected(_endpoint, _ok) => {},
                        NetEvent::Accepted(_endpoint, _listener) => {}
                        NetEvent::Disconnected(_endpoint) => {println!("Client disconnected!");},
                        NetEvent::Message(_, data) => {
                            let message = serde_json::from_slice::<Message>(data).unwrap();
                            println!("Message: {}. From: {}", message.data, message.sender_id);
                            Node::store_message(message);
                        },
                    },
                    NodeEvent::Signal(_) => {}
                });
            }
        });
    }

    fn store_message(message: Message)
    {
        let mut messages_mutex = MESSAGE_QUEUE.lock().unwrap();
        let messages = &mut *messages_mutex;
        messages.push(message); 
    }

    fn stop_receiver(&self)
    {
        self.handler.stop();
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct SendReq {
    msg: String,
    receiver_id: String,
    sender_id: String
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
enum Args
{
    /// Start a receiver with the given name on a given port.
    Start {port: String},
}

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string()
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let node = match args {
        Args::Start { port } => {
            println!("Starting a node with receiver listening on {port}.");
            Node::new(&port)
        }
    };

    loop {
        println!("Enter send(s) to send message OR exit(e) to exit");
        let input = prompt("> ");
        if input.to_lowercase() == "send" || input.to_lowercase() == "s" {
            let message = prompt("message: ");
            let receiver_port = prompt("receiver's port: ");

            node.send(&receiver_port, message.as_str());
        } 
        else if input == "exit" || input == "e" { 
            node.stop_receiver();
            break; 
        } else {
            println!("invalid input retry...");
        };
    }
}
