import produce from "immer"
const express = require('express')
var hashmap = require('hashmap');
var assert = require('assert');
const app = express()

class ClientConnection {
  constructor(uuid) {
    this.uuid = uuid;
    this.creation_time = Date.now();
    this.last_heartbeat_time = Date.now();
  }
}

const current_clients = new hashmap();

app.get("/heartbeat", function(req, res) {
  uuid = req.query.uuid;
  assert(uuid);
  existing_client = current_clients.get(uuid);
  if(existing_client) {
    existing_client.last_heartbeat_time = Date.now();
    current_clients.set(uuid, existing_client);
  } else {
    t = new ClientConnection(uuid)
    current_clients.set(uuid, t);
  }
  res.send("acknowledged");
});
app.get("/create_trap", function(req, res) {

});
app.get("/destroy_trap", function(req, res) {

});
app.listen(3000);
