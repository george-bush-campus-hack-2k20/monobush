import React from "react";
import "./App.css";

import axios from "axios";
import { v4 as genUUID } from "uuid";

import { Spinner } from "./pages/Spinner";
import { Trap } from "./pages/traps";

var client = axios.create({
  baseURL: "http://18.130.245.224/",
  timeout: 1000
});

class App extends React.Component {
  state = {
    clientStage: "waiting",
    heartbeatInterval: null,
    trap: null,
    uuid: genUUID()
  };

  componentDidMount = () => {
    this.setState({
      heartbeatInterval: setInterval(this.sendHeartbeat, 100)
    });
  };

  sendHeartbeat = async () => {
    let data;
    try {
      const response = await client.post("/client/heartbeat", {
        id: this.state.uuid
      });
      console.log(response);
      data = response.data;
    } catch (err) {
      this.forceUpdate();
    }
    if (data.state === "ready") {
      this.setState({
        clientStage: "playing",
        trap: {
          type: data.trap,
          color: data.color,
          text: data.text
        }
      });
    }
    if (data.state === "heartbeat" && !data.ok) {
      this.forceUpdate();
    }
  };

  renderPage = () => {
    switch (this.state.clientStage) {
      case "waiting":
        return <Spinner />;
      case "playing":
        return (
          <Trap
            userId={this.state.uuid}
            client={client}
            trapData={this.state.trap}
          />
        );
      default:
        console.err("Something went terribly wrong...");
    }
  };

  render() {
    return (
      <div className="App">
        <header className="App-header">{this.renderPage()}</header>
      </div>
    );
  }
}

export default App;
