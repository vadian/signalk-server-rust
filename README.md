# SignalK Reference Server - Rust Edition

This is a pre-alpha work in progress.  This project may split into separate repositories in the future.
The purpose of this project is to implement the SignalK server specification found at:
https://signalk.org/specification/1.7.0/doc/

The project will focus first on implementing the SignalK data model, and then REST API functionality 
including historical lookup.  Websocket delta subscription will then be implemented.

Long-term, Deno is being evaluated as a JavaScript runtime for plugin support, with the goal being
plugin support parity with the Node reference implementation.

This project is designed to work with Influx IOx, which is available via Docker.
The axum/tokio stack will be used for multithreading and web hosting.

Time series data will be recorded, and requests will be event sourced from the most recent snapshot

Note: Requires LLDB plugin for vscode to debug

License: MIT, see LICENSE