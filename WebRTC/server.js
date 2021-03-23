const fs = require("fs");
const express = require("express");
const app = express();
const http = require("http");
const { v4: uuidV4 } = require("uuid");
const privateKey = fs.readFileSync(__dirname + "/selfsigned.key");
const privateCert = fs.readFileSync(__dirname + "/selfsigned.crt");
const server = require("https").Server(
  { key: privateKey, cert: privateCert },
  app
);
const io = require("socket.io")(server);
const PORT = 443;
const PEER_PORT = 3001;
const { PeerServer } = require("peer");

app.set("view engine", "ejs");
app.use(express.static("public"));

app.get("/", (req, res) => {
  res.redirect(`/${uuidV4()}`);
});

app.get("/:room", (req, res) => {
  res.render("room", { roomId: req.params.room });
});

const peerServer = PeerServer(
  {
    port: PEER_PORT,
    path: "/",
    ssl: {
      key: privateKey,
      cert: privateCert,
    },
  },
  () => console.log("Running peer server on port " + PEER_PORT)
);

io.on("connection", (socket) => {
  socket.on("join-room", (roomId, userId) => {
    socket.join(roomId);
    setTimeout(() => {
      socket.to(roomId).emit("user-connected", userId);
      // when is loads too fast, the camera wont load in
    }, 1500);

    setTimeout(() => {
      socket.on("disconnect", () => {
        socket.to(roomId).emit("user-disconnected", userId);
        // when it is disconnected too fast it wont remove in time
      });
    }, 1000);
  });
});

server.listen(PORT, () => {
  console.log("Server is running on port " + PORT);
});

// Redirect from http port 80 to https
http
  .createServer((req, res) => {
    res.writeHead(301, {
      Location: "https://" + req.headers["host"] + req.url,
    });
    res.end();
  })
  .listen(80);
