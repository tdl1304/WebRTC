# WebRTC & STUN-Server 
[Last CI run](https://github.com/tdl1304/WebRTC/actions/runs/680292306)  
## Introduction
This is my solution to an optional assignment in the subject IDATT2104 at NTNU Trondheim. 
The solution includes a non-fully implemented STUN-Server in rustlang and a WebRTC program, that utilizes this STUN-Server. 
The WebRTC program uses peerjs, and stun server is currently set to my own stun server.  

*To edit this change Peer parameteres in file: project-folder/WebRTC/public/script.js*  

This readme will only cover the STUN-Server, however instructions for starting the WebRTC program is included.  

## Implemented functionality  
* Incomming TCP and UDP connections
* Binding requests (type: 0x0001)
    * XOR-MAPPED-ADDRESS (type: 0x0020)
    * Supports IPv4
    * Supports IPv6
    
* Error responses
    * Unknown attributes (code: 420)
    * Other error codes
    
* Multithreading

## Limitations and further work
* Cannot respond to any requests other than binding requests
    * Especially request with identification, it will just send xor mapped address as response
    
* No support for encrypted endpoint protocols (SSL/TSL)

## External Dependencies
* [Bincode](https://github.com/bincode-org/bincode)
    * A compact encoder / decoder pair that uses a binary zero-fluff encoding scheme. 
      The size of the encoded object will be the same or smaller than the size that the object takes up in memory in a running Rust program.
    
    * Used for serializing structs into byte arrays
* [Serde](https://docs.serde.rs/serde/index.html) 
    * Serde is a framework for serializing and deserializing Rust data structures efficiently and generically. 
    
    * Used in conjunction with Bincode to serialize structs into byte arrays
    
## Installation & How To Run
*For development*
### STUN-Server
1. After cloning repository through terminal, or directly through an IDE. 
   Either run main.rs with IDE or use terminal while inside project repository:
    ```rust
    cargo build
    cargo run
    ```
2. Stun server is now running on port 3478. Make sure to configure firewall and NAT portforwarding for use.
### WebRTC
1. Set working repository to WebRTC folder.
2. Enter into terminal:
    ````
    npm ci
   # with nodemon
   npm start devStart
   # alternatively with node
   node server.js
    ````
3. Server is now running on port 443 and 80, with a peer server running on port 3001.

## Running tests
To run tests, run command:
````
cargo test
````
## API documentation
Clone project and open index.html file: project-folder/docs/stun-server/index.html  
Or run in terminal:
````
cargo doc --open
````
