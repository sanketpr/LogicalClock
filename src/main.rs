#[allow(unused_imports)]
mod lamport_clock;
mod event;

use lamport_clock::{LamportClock};
use event::{Event, Message, EventType};
use warp::Filter;
use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

static _NODES_MAP: Lazy<Mutex<HashMap<String, Node>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug)]
struct Node {
    id: String,
    addr: SocketAddr,
    clock: LamportClock
}

impl Node {
    fn new(id: &str, addr: SocketAddr) -> Self {
        let clock = LamportClock { time_stamp: 0 };
        Node {
            id: id.to_string(),
            addr,
            clock
        }
    }
    
    fn send(&mut self, reciever_id: &str, data: String) {
        let clock = &self.clock;

        let message = Message {
            sender_id: self.id.clone(),
            reciever_id: reciever_id.to_string(),
            data,
            time_stamp: clock.time_stamp
        };

        let event = Event{
            id: "123".to_string(),
            r#type: EventType::Send,
            message: Some(message) 
        };

        self.clock.process_event(event);

        // SEND MESSAGE
    }

    fn accept(&mut self, message: Message) {
        let event = Event {
            id: nanoid!(),
            r#type: EventType::Recieve,
            message: Some(message)
        };

        self.clock.process_event(event);
    }

    fn time_event(node_id: String) {
        std::thread::spawn(move || {
            loop {
                {
                    let nodes = &mut *_NODES_MAP.lock().unwrap();
                    let mut node = nodes.remove(&node_id).unwrap(); 
                    let event = Event {
                        id: nanoid!(),
                        r#type: EventType::Local,
                        message: None
                    };
                    node.clock.process_event(event);
                    nodes.insert(node_id.clone(), node);
                };
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }); 
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct SendReq {
    msg: String,
    receiver_id: String,
    sender_id: String
}

async fn handle_recieve_req(message: Message) -> Result<impl warp::Reply, warp::Rejection>  {
    let nodes = &mut *_NODES_MAP.lock().unwrap();
    let mut node = nodes.remove(&message.reciever_id).unwrap();
    node.accept(message);
    nodes.insert(node.id.clone(), node);
    Ok(warp::reply::with_status(
        "Received message",
        http::StatusCode::CREATED,
    ))
}

async fn handle_send_req(req: SendReq) -> Result<impl warp::Reply, warp::Rejection>  {
    let nodes = &mut *_NODES_MAP.lock().unwrap();
    let mut node = nodes.remove(&req.receiver_id).unwrap();
    node.send(&req.receiver_id, req.msg);

    nodes.insert(node.id.clone(), node);

    Ok(warp::reply::with_status(
        "Processed send request",
        http::StatusCode::OK,
    ))
}

fn receive_msg_req_body() -> impl Filter<Extract = (Message,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn send_msg_req_body() -> impl Filter<Extract = (SendReq,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn node_listen(addr: SocketAddr) {

    let accept_message_req = warp::post()
        .and(warp::path("accept_message"))
        .and(warp::path::end())
        .and(receive_msg_req_body())
        .and_then(handle_recieve_req);

    
    let send_message_req = warp::post()
        .and(warp::path("send_message"))
        .and(warp::path::end())
        .and(send_msg_req_body())
        .and_then(handle_send_req);
    
    let routes = send_message_req.or(accept_message_req);
    println!("Server listening on {:#?}", addr);

    warp::serve(routes)
        .run(addr)
        .await;

    println!("ERROR. Server stopped listening.");
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::from_str("127.0.0.1").unwrap(), 3032);

    let node_1 = Node::new("Abc",addr);
    let node_2 = Node::new("Bcd",addr);
    Node::time_event(node_1.id.clone());
    Node::time_event(node_2.id.clone());

    {
        let nodes = &mut *_NODES_MAP.lock().unwrap();
        nodes.insert(node_1.id.clone(), node_1);
        nodes.insert(node_2.id.clone(), node_2);
    };

    node_listen(addr).await;

}
