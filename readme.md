# pods slave node
This is part of [pods](https://github.com/clowzed/pods-master) project

![Docker Image Size (latest by date)](https://img.shields.io/docker/image-size/clowzed/pods-slave?color=u&label=docker%20image%20size)     ![GitHub](https://img.shields.io/github/license/clowzed/pods-slave?color=g)
[![build](https://github.com/clowzed/pods-slave/actions/workflows/build.yml/badge.svg)](https://github.com/clowzed/pods-slave/actions/workflows/build.yml)


## Installation
Before starting this node you need be sure that master node with service registry is running

- Get access token from master node. Use settings endpoint to get it or make request to service registry.

1) Use docker-compose
```sh
# modify docker-compose.yml
sudo docker compose up -d
```

2) Direct running
```sh
# create .env file
cargo run --release
```

## Environment
| VARIABLE         | DESCRIPTION                      |
|------------------|----------------------------------|
| PORT             | Sets port for listenong          |
| UPLOAD_FOLDER    | Folder to save uploaded episodes |
| SERVICE_REGISTRY | Service registry url             |
| ACCESS_TOKEN     | Service registry access token    |
