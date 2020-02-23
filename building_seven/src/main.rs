#[macro_use] extern crate nickel;

use crate::nickel::{Nickel, HttpRouter, QueryString, status::StatusCode, MediaType};
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use chrono::{Utc, DateTime};
use uuid::Uuid;
use log::{info, trace, warn, debug};
use crate::nickel::JsonBody;
use fern;
use serde_derive::{Serialize, Deserialize};
mod logging;



struct UserSession {
    created_at: DateTime<Utc>,
    last_heartbeat: DateTime<Utc>,
    uuid: Uuid
}
impl UserSession {
    pub fn new(in_uuid: Uuid) -> UserSession {
	UserSession {
	    created_at: Utc::now(),
	    last_heartbeat: Utc::now(),
	    uuid: in_uuid
	}
    }
}

struct Trap {
}

#[derive(Serialize, Deserialize)]
struct ClientPayload { id: String }
fn main() {
    logging::setup();
    info!("Logger configured");

    let mut server = Nickel::new();
    info!("Server obj initialised");
    let c_users: Arc<RwLock<HashMap<Uuid, Mutex<UserSession>>>> = Arc::new(RwLock::new(HashMap::new()));
    info!("User mapping initialised");
    let trap_pool: RwLock<Vec<Trap>> = RwLock::new(Vec::new());
    let trap_allocated: RwLock<HashMap<Uuid, Trap>> = RwLock::new(HashMap::new());
    info!("Trap pool(s) initialised");
    server.post("/heartbeat", middleware! { |request, mut response| {
	info!("Got heartbeat request!");
	response.set(StatusCode::NoContent);
	response.set(MediaType::Json);
	let client = try_with!(response, request.json_as::<ClientPayload>().map_err(|e| (StatusCode::BadRequest, e)));
	let uuid_str = client.id;
	debug!("Processing UUID: {}", uuid_str);
	let uuid = Uuid::parse_str(&uuid_str).unwrap();
	debug!("Successfully created UUID object");
	let mut c_users_write = c_users.write().unwrap();
	debug!("Got c_users read handle");
	let mut theoretical_user_session = c_users_write.get(&uuid);
	debug!("Requested user session for UUID: {}", uuid_str);
	if theoretical_user_session.is_some() {
	    // we can update their timestamp
	    debug!("Session already exists!");
	    let mut user_write_session = c_users_write.get_mut(&uuid).unwrap().lock().unwrap();
	    debug!("Previous heartbeat for {} was {}", user_write_session.uuid, user_write_session.last_heartbeat);
	    user_write_session.last_heartbeat = Utc::now();
	} else {
	    // create a new one
	    debug!("No session found, creating one...");
	    let user_session = UserSession::new(uuid);
	    c_users_write.insert(uuid, Mutex::new(user_session));
	    debug!("Wrote new user session");
	}
	response.set(StatusCode::Accepted);
	"hi"
    }});
    // uuid, type, color, name


    // 200 if I have a trap
    // 204 if none avaliable
    server.post("/request_trap", middleware! { |request, mut response| {
	response.set(MediaType::Json);
	let client = try_with!(response, request.json_as::<ClientPayload>().map_err(|e| (StatusCode::BadRequest, e)));
	let uuid = Uuid::parse_str(&client.id).unwrap();
	if !trap_pool.read().unwrap().is_empty() {
	    // we have a trap to allocate
	    // make sure they haven't already been allocated one
	    let mut trap_allocated_handle = trap_allocated.write().unwrap();
	    if trap_allocated_handle.contains_key(&uuid) {
		// fuck off greedy bastard you already have a trap
		return response.send("fuck right off");
	    } else {
		// you can have one
		let mut trap = trap_pool.write().unwrap().pop().unwrap();
		trap_allocated_handle.insert(uuid, trap);
		response.set(StatusCode::Accepted);
	    }
	}
    }});
    
    info!("Endpoints configured");
    server.listen("127.0.0.1:3000").unwrap();
}
