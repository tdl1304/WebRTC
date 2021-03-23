const socket = io("/");
const videoGrid = document.getElementById("video-grid");
const peers = {};
const id = [];
let rootId = "root";
const myVideo = document.createElement("video");
const myPeer = new Peer(undefined, {
  config: {
    iceServers: [
      { url: "stun:84.210.210.181:3478" },
      {
        url: "turn:numb.viagenie.ca",
        credential: "muazkh",
        username: "webrtc@live.com",
      },
    ],
  },
  secure: true,
  host: "/",
  port: "3001",
  path: "/",
});

myPeer.on("open", (id) => {
  socket.emit("join-room", ROOM_ID, id);
});

socket.on("user-disconnected", (userId) => {
  console.log("disconnected: " + userId);
  if (peers[userId]) {
    peers[userId].close();
  } else if (peers[rootId]) {
    peers[rootId].close();
  } else {
    console.log("Something wrong");
  }
});

function init() {
  navigator.mediaDevices
    .getUserMedia({
      video: true,
      audio: true,
    })
    .then((stream) => {
      //Add own video stream
      addVideoStream(myVideo, stream);
      //Listen for calls, then answer
      myPeer.on("call", (call) => {
        call.answer(stream);
        const video = document.createElement("video");
        call.on("stream", (userVideoStream) => {
          addVideoStream(video, userVideoStream);
        });

        call.on("close", () => {
          video.remove();
        });
        if (!peers[rootId]) peers[rootId] = call;
      });
      //Signal new user connected
      socket.on("user-connected", (userId) => {
        connectToNewUser(userId, stream);
      });
    })
    .catch((error) => {
      console.log(error);
    });
}

function connectToNewUser(userId, stream) {
  const call = myPeer.call(userId, stream);
  const video = document.createElement("video");
  call.on("stream", (userVideoStream) => {
    addVideoStream(video, userVideoStream);
  });
  call.on("close", () => {
    video.remove();
  });

  peers[userId] = call;
}

function addVideoStream(video, stream) {
  video.srcObject = stream;
  video.addEventListener("loadedmetadata", () => {
    video.play();
  });
  videoGrid.append(video);
}

function main() {
  myVideo.muted = true;
  init();
}

main();
