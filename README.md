# InfraRMM
CLI Based RMM Built in Rust

## Goal:
* Create a simple interface for System Administrators to manage remote systems via the CLI.
* Automate basic system functions.
* Be able to send a client executable to a client for troubleshooting without the need for SSH Credentials - allows the client to be in control of their System Administrator(s).

## Roadmap:
* Introduce security either via OpenSSL keys or RSA keys.
* Introduce automation menu where bash/batch/powershell scripts can be streamed and ran through the client service.
* Introduce an SSH like interactive terminal after selecting a client.
* Pipe command responses and errors back to the server service.
* Introduce a monitoring menu for each client, will be able to have threshold based alerts.
* Introduce a monitoring dashboard that shows every connected client. May be split between pages or an auto-revolving screen to cover all clients.
