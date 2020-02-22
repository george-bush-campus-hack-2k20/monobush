#[macro_use] extern crate nickel;

use nickel::Nickel;
use crate::nickel::{HttpRouter, QueryString, status::StatusCode};
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use chrono::{Utc, DateTime};
use uuid::Uuid;


struct UserSession {
    created_at: DateTime<Utc>,
    last_heartbeat: DateTime<Utc>,
    uuid: Uuid
}

struct Trap {
}

fn main() {
    let mut server = Nickel::new();
    let mut c_users: Arc<RwLock<HashMap<Uuid, Mutex<UserSession>>>> = Arc::new(RwLock::new(HashMap::new()));
    {
	let mut c_users_endpoint = c_users.clone();

	let mut trap_pool: RwLock<Vec<Trap>> = RwLock::new(Vec::new());
	let mut trap_allocated: RwLock<HashMap<Uuid, Trap>> = RwLock::new(HashMap::new());
	server.get("/heartbeat", middleware! { |request, mut response| {
	    let query = request.query();
	    let uuid_str = query.get("uuid").expect("No UUID parameter");
	    let uuid = Uuid::parse_str(uuid_str).unwrap();
	    if c_users_endpoint.read().unwrap().get(&uuid). {
		
	    }
	}});
	// uuid, type, color, name


	// 200 if I have a trap
	// 204 if none avaliable
	server.get("/request_trap", middleware! { |request, mut response| {
	    let query = request.query();
	    let uuid_str = query.get("uuid").expect("No UUID parameter");
	    let uuid = Uuid::parse_str(uuid_str).unwrap();
	    if !trap_pool.read().unwrap().is_empty() {
		// we have a trap to allocate
		// make sure they haven't already been allocated one
		let mut trap_allocated_handle = trap_allocated.write().unwrap();
		if trap_allocated_handle.contains_key(&uuid) {
		    // fuck off greedy bastard you already have a trap
		    response.send("fuck right off");
		} else {
		    // you can have one
		    let mut trap = trap_pool.write().unwrap().pop().unwrap();
		    trap_allocated_handle.insert(uuid, trap);
		}
	    } else {
		response.error(StatusCode::NoContent, "traps are gay :(");
	    }
	}});
    }
    server.listen("127.0.0.1:3000");
}
