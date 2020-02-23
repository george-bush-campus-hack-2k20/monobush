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
use serde_json;
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

#[derive(Serialize, Deserialize)]
struct Trap {
    id: String,
    state: String,
    trap: String,
    color: String,
    text: String
}


const empty_trap: Trap = Trap {
    id: String::new(),
    state: String::new(),
    trap: String::new(),
    color: String::new(),
    text: String::new()
};

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
    let trap_total: RwLock<HashMap<Uuid, Trap>> = RwLock::new(HashMap::new());
    info!("Trap pool(s) initialised");
    server.post("/client/heartbeat", middleware! { |request, mut response| {
	info!("Got heartbeat request!");
	response.set(StatusCode::NoContent);
	response.set(MediaType::Json); let client = try_with!(response, request.json_as::<ClientPayload>().map_err(|e| (StatusCode::BadRequest, e)));
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

    server.post("/client/activate_trap", middleware! { |request, mut response| {
    }});

    server.post("/game/create_trap", middleware! { |request, mut response| {
	// check the req
	let trap_req = try_with!(response, request.json_as::<Trap>().map_err(|e| (StatusCode::BadRequest, e)));
	response.set(MediaType::Json);
	let mut trap_pool_write = trap_pool.write().unwrap();
	trap_pool_write.push(trap_req);
	response.set(StatusCode::Accepted);
    }});

    server.post("/game/destroy_trap", middleware! { |request, mut response| {
    }});

    server.get("/game/trap_status/:id", middleware! { |request, mut response| {
	response.set(MediaType::Json);
	let con = trap_total.read().unwrap();
	let x = match request.param("id") {
	    Some(id) => {
		let uuid_id = Uuid::parse_str(id).unwrap();
		con.get(&uuid_id)
	    },
	    None => None
	}.unwrap();
	serde_json::to_value(x).map_err(|e| (StatusCode::InternalServerError, e))

    }});

    // 200 if I have a trap
    // 204 if none avaliable
    server.post("/client/request_trap", middleware! { |request, mut response| {
	response.set(MediaType::Json);
	let client = try_with!(response, request.json_as::<ClientPayload>().map_err(|e| (StatusCode::BadRequest, e)));
	let uuid = Uuid::parse_str(&client.id).unwrap();
	let mut trap_pool_write = trap_pool.write().unwrap();
	match trap_pool_write.is_empty() {
	    true => {
		let mut trap_alloc_write = trap_allocated.write().unwrap();
		let trap = trap_pool_write.pop().unwrap();
		trap_alloc_write.insert(uuid, trap);
	    }
	    _ => (),
	}
    }});
    
    info!("Endpoints configured");
    server.listen("127.0.0.1:3000").unwrap();
}
