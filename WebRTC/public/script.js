const socket = io("/");
const videoGrid = document.getElementById("video-grid");
const peers = {};
let hasVideo = false;
let hasAudio = false;
let myId = 0;
const myVideo = document.createElement("video")
const myPeer = new Peer(undefined, {
  config: {
    iceServers: [
      { url: "stun:84.210.213.159:3478" },
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
  myId = id;
  socket.emit("join-room", ROOM_ID, id);
});

socket.on("user-disconnected", async (userId) => {
  if (peers[userId]) {
    console.log("User disconnected: " + userId);
    await peers[userId].close();
  }
});

async function init() {
  await navigator.mediaDevices.enumerateDevices().then(async devices=> {
    await devices.forEach(device=>{
      if(device.kind === "audioinput") {hasAudio = true;}
      else if (device.kind === "videoinput") {hasVideo = true;}
    })
  })

  navigator.mediaDevices
    .getUserMedia({
      video: hasVideo,
      audio: hasAudio,
    })
    .then((stream) => {
      //Add own video stream
      if(!hasVideo) {
        videoGrid.append(myVideo)
      } else {
        addVideoStream(myVideo, stream);
      }
      //Listen for calls, then answer
      myPeer.on("call", async (call) => {
        call.answer(stream);
        myPeer.on('connection', function(conn) {
          conn.on('data', function(data){
            peers[data] = call
          });
        });
        const video = document.createElement("video");
        call.on("stream", async (userVideoStream) => {
          await addVideoStream(video, userVideoStream);
        });

        call.on("close", () => {
          video.remove();
        });

      });
      //Signal new user connected
      socket.on("user-connected", async (userId) => {
        await connectToNewUser(userId, stream);
      });
    })
    .catch((error) => {
      console.log(error);
    });
}

async function connectToNewUser(userId, stream) {
  const call = myPeer.call(userId, stream);
  const conn = myPeer.connect(userId);
  conn.on('open', () =>{
    conn.send(myId)
  })
  const video = document.createElement("video");
  call.on("stream", async (userVideoStream) => {
    await addVideoStream(video, userVideoStream);
  });
  call.on("close", () => {
    video.remove();
  });
  console.log("User connected: "+userId)
  peers[userId] = call;
}

function addVideoStream(video, stream) {
  video.srcObject = stream;
  video.autoplay = true;
  video.load();
  videoGrid.append(video)
}

async function main() {
  myVideo.muted = true;
  await init();
}

main();
