import React from "react";
import "./App.css";

import { Spinner } from "./pages/Spinner";
import { Trap } from "./pages/Trap"

class App extends React.Component {
  state = {
    clientStage: "waiting",
    trap: null
  };

  componentDidMount = () => {
    setTimeout(() => {
      this.setState({
        clientStage: "playing",
        trap: {
          type: "level",
          color: "#ffffff",
          text: "Text"
        }
      });
    }, 5000);
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
