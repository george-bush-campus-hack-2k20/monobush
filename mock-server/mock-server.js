const app = require("express")();
const cors = require("cors")

app.use(cors())

app.post('/client/heartbeat', (req, res) => {
  res.json({
    state: "ready",
    trap: "spikes",
    color: "#ffffff",
    text: "Hello there"
  })
})

app.listen("8080", () => console.log("App listening on port 8080"));
