#![allow(dead_code,unused_imports)]
#[macro_use] extern crate nickel;
use hyper::header::{AccessControlAllowOrigin, AccessControlAllowHeaders};
use hyper::method::Method;
use nickel::{Request, Response, MiddlewareResult};
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

use std::iter::FromIterator;
use std::thread;

mod logging;


const MAX_HEARTBEAT_INTERVAL: i64 = 2500; // in ms
const HEARTBEAT_CULL_INTERVAL: u64 = 100;  // "

struct UserSession {
    created_at: DateTime<Utc>,
    last_heartbeat: DateTime<Utc>,
    uuid: String
}
impl UserSession {
    pub fn new(in_uuid: &str) -> UserSession {
	UserSession {
	    created_at: Utc::now(),
	    last_heartbeat: Utc::now(),
	    uuid: (*in_uuid).to_string()
	}
    }
}

// any request that only takes a uuid uses the following
#[derive(Serialize, Deserialize)]
struct UuidRequest {
    id: String,
}
#[derive(Serialize, Deserialize)]
struct Trap {
    id: String,
    state: String,
    trap: String,
    color: String,
    text: String
}

fn enable_cors<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    // Set appropriate headers
    res.set(AccessControlAllowOrigin::Any);
    res.set(AccessControlAllowHeaders(vec![
        // Hyper uses the `unicase::Unicase` type to ensure comparisons are done
        // case-insensitively. Here, we use `into()` to convert to one from a `&str`
        // so that we don't have to import the type ourselves.
        "Origin".into(),
        "X-Requested-With".into(),
        "Content-Type".into(),
        "Accept".into(),
    ]));

    // Pass control to the next middleware
    res.next_middleware()
}

fn main() {
    logging::setup().unwrap();
    // maps TRAPUUIDs to TRAPS
    let trap_map_master: Arc<Mutex<HashMap<String, Trap>>> = Arc::new(Mutex::new(HashMap::new()));
    // maps USERUUIDS to USERSESSIONS
    let users_master: Arc<Mutex<HashMap<String, UserSession>>> = Arc::new(Mutex::new(HashMap::new()));

    // maps USERUUIDS TO TRAPUUIDS
    let user_trap_map_master: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    {
	let users = users_master.clone();
	let user_trap_map = user_trap_map_master.clone();
	thread::spawn(move || { loop {
	    debug!("Entering cull loop...");
	    {
		let c_time = Utc::now().timestamp_millis();
		let mut users_lock = users.lock().unwrap();
		let mut user_trap_map_lock = user_trap_map.lock().unwrap();
		let mut to_remove_queue = Vec::new();
		for user in users_lock.iter().map(|x| x.1) {
		    debug!("Considering UUID {}", user.uuid);
		    if c_time - user.last_heartbeat.timestamp_millis() > MAX_HEARTBEAT_INTERVAL {
			// remove them
			to_remove_queue.push(user.uuid.clone());
		    }
		}
		debug!("UUIDs to be culled: {:?}", to_remove_queue);
		for to_remove in to_remove_queue {
		    debug!("Deleting UUID {}", to_remove);
		    users_lock.remove(&to_remove);
		    user_trap_map_lock.remove(&to_remove);
		}
	    }
	    thread::sleep(std::time::Duration::from_millis(HEARTBEAT_CULL_INTERVAL));
	}});
    }
    let mut server = Nickel::new();
    server.utilize(enable_cors);

    {
	let trap_map = trap_map_master.clone();
	let user_trap_map = user_trap_map_master.clone();
	let users = users_master.clone();
	server.post("/client/heartbeat", middleware! { |request, mut response| {
	    response.set(MediaType::Json);
	    // make sure they gave us a uuid
	    let client = try_with!(response, request.json_as::<UuidRequest>().map_err(|e| (StatusCode::BadRequest, e)));
	    assert!(Uuid::parse_str(&client.id).is_ok());
	    // check I'm aware of them
	    let mut users_lock = users.lock().unwrap();
	    let users_that_match_uuid = users_lock.iter().filter(|x| x.0 == &client.id);
	    match users_that_match_uuid.count() {
		1 => {
		    // one user, yay
		    // do they have a trap already?
		    let mut users_trap_map_lock = user_trap_map.lock().unwrap();
		    if users_trap_map_lock.get(&client.id).is_none() {
			// we can assign them one
			// unallocated traps
			let trap_map_lock = &trap_map.lock().unwrap();
			let mut valid_traps = trap_map_lock.iter().filter(|x| users_trap_map_lock.get_key_value(x.0).is_none());
			match valid_traps.next() {
			    Some(trap_id_trap) => {
				// now create the user <-> trap mapping
				users_trap_map_lock.insert(client.id, trap_id_trap.0.to_string());
				return response.send(serde_json::to_string(trap_id_trap.1).unwrap());
			    },
			    None => {
				return response.send("{ \"state\": \"waiting\"}");
			    }
			};
		    }
		    // they already had a trap, just return them it again
		    let trap_id_currently_assigned = users_trap_map_lock.get(&client.id).unwrap(); // unwrap is okay because code path is only followed if not none
		    let trap_map_lock = trap_map.lock().unwrap();
		    let trap_from_id = trap_map_lock.get(trap_id_currently_assigned).unwrap();
		    return response.send(serde_json::to_string(trap_from_id).unwrap());
		}
		0 => {
		    // they didn't exist before now, just create them
		    users_lock.insert(client.id.clone(), UserSession::new(&client.id));
            return response.send("{ \"state\": \"waiting\"}");
		}
		_ => ()
	    };
	    // no clue, more than one UUID?
	    response.set(StatusCode::NoContent);
	    ""
	}});
    }


    {
	let trap_map = trap_map_master.clone();
	server.post("/game/create_trap", middleware! { |request, mut response| {
	    response.set(MediaType::Json);
	    response.set(StatusCode::NotFound);
	    // we are expecting a well-formed trap object
	    let client = try_with!(response, request.json_as::<Trap>().map_err(|e| (StatusCode::BadRequest, e)));
	    assert!(Uuid::parse_str(&client.id).is_ok());
	    // looks good, fine, add it to the traps
	    let mut trap_map_lock = trap_map.lock().unwrap();
	    trap_map_lock.insert(client.id.clone(), client);
	    response.set(StatusCode::from_u16(200));
	    ""
	}});
    }

    {
	let trap_map = trap_map_master.clone();
	let user_trap_map = user_trap_map_master.clone();
	server.post("/game/destroy_trap", middleware! { |request, mut response| {
	    response.set(MediaType::Json);
	    response.set(StatusCode::NotFound);
	    let client = try_with!(response, request.json_as::<UuidRequest>().map_err(|e| (StatusCode::BadRequest, e)));
	    assert!(Uuid::parse_str(&client.id).is_ok());
	    let mut trap_map_lock = trap_map.lock().unwrap();
	    trap_map_lock.remove(&client.id);
	    // also remove it if it's in the userid <-> trapid mapping
	    let uuid_to_remove = {
		let user_trap_map_lock = user_trap_map.lock().unwrap();
		let t_vec: Vec<&String> = user_trap_map_lock.iter().filter(|x| x.1 == &client.id).map(|x| x.0).collect();
		t_vec[0].clone()
	    };
	    let mut user_trap_map_lock = user_trap_map.lock().unwrap();
	    user_trap_map_lock.remove(&uuid_to_remove);
	    response.set(StatusCode::from_u16(200));
	}});
    }
    server.listen("127.0.0.1:8080").unwrap();
}
