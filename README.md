# task-multiplayer-fps
grit:lab Åland Islands 2023  

![demo](demo.png)  

## Description
The retro game "Maze Wars" reimplementation using the [rust](https://www.rust-lang.org).  
For details see [task and audit questions](https://github.com/01-edu/public/tree/master/subjects/multiplayer-fps).  


## Requirements
- Properly installed [rust](https://www.rust-lang.org). Version `1.74.1` used for development.  


## How to run

### Build `hybrid` executable for server and client
terminal: `./do`  

### Run server
terminal: `./hybrid server`  

### Run client
terminal: `./hybrid client`  

**Use the same ip:port pair for server and clients.**  

## Development run the server and/or client

### Build release executable for server and run
terminal: `./dev-server`  

### Build release executable for client and run
terminal: `./dev-client`  

## How to test between different computers
### Computer 1
**(This computer should be visible on the network to the clients)**

### Run server
terminal: `./hybrid server`  

Type :  127.0.0.1:8000  
Name : Unique name (Don’t use same on client and server)  

### Run client
terminal: `./hybrid client`  

Type :  127.0.0.1:8000  
Name : Unique name (Don’t use same on client and server)  

### Find out your network IP. Computer 2 will need it.  
(example of network IP: 10.5.126.33)  

terminal: `ipconfig getifaddr en1`  

### Computer 2
### Build `hybrid` executable for server and client
terminal: `./do`  

### Run client  
terminal: `./hybrid client`  
Type : network IP from Computer 1 , with port. Example :  
10.5.126.33:8000  

Name : Unique Name (Don’t use same on client and server)  

### Game levels 
Game begins on level 1 by default. With the exception of :  
If port number ends with 2, game will start with level 2 map.  
If port number ends with 3, game will start with level 3 map.   
#### Example:
terminal: `./hybrid server`  

Type :  127.0.0.1:8002 (level 2)   
Type :  127.0.0.1:8003 (level 3)  

### Instructions on gameplay 
- Use arrow keys to move around.
- Use Space bar to shoot.
- Enemy should be outside of spawn cell to be eliminated by a shot.

## Authors
- [blueskiy01](https://github.com/blueskiy01)  
- [healingdrawing](https://healingdrawing.github.io/)  
