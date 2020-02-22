import React from "react";
import "./App.css";

import { Spinner } from "./pages/Spinner";
import { Trap } from "./pages/Trap";

class App extends React.Component {
  state = {
    clientStage: "waiting",
    heartbeatInterval: null,
    trap: null
  };

  componentDidMount = () => {
    setTimeout(() => {
      this.setState({
        clientStage: "playing",
        heartbeatInterval: setInterval(this.sendHeartbeat, 100),
        trap: {
          type: "level",
          color: "#ffffff",
          text: "Text"
        }
      });
    }, 5000);
  };

  sendHeartbeat = () => {
    console.log("This is a heartbeat");
  };

  renderPage = () => {
    switch (this.state.clientStage) {
      case "waiting":
        return <Spinner />;
      case "playing":
        return <Trap trapData={this.state.trap} />;
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
