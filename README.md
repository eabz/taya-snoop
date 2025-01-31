<h1 align="center">
<strong>Taya Snoop</strong>
</h1>
<p align="center">
<strong>Taya logs and event indexer based on ETH Snoop</strong>
</p>

![build](https://github.com/eabz/eth-snoop/actions/workflows/build.yml/badge.svg)


## Build

You can try the indexer locally or using Docker.

1. Clone the repository

```
git clone https://github.com/eabz/eth-snoop && cd eth-snoop
```

2. Build the program

```
cargo build --release
```

## Docker

Build the docker image
```
docker build . -t snoop
```

Copy the `.env.example` and rename it to `.env `

Start the indexer
```
docker compose up
```

## Program flags

| Flag           |  Default  | Purpose                                                                      |
| -------------- | :-------: | -----------------------------------------------------------------------------|
| `--rpc`        |  `empty`  | URL of the RPC endpoint to fetch chain data and logs.                        |
| `--database`   |  `empty`  | MongoDB connection URL (e.g. 'mongodb://user:password@host:27017/dbname').   |
| `--batch-size` |   `50`    | Number of blocks to fetch in each batch of logs.                             |
