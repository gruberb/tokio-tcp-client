# tokio-tcp-client
A helper client to test my protohackers.com server implementations

## Install

```bash
$ git clone git@github.com:gruberb/tokio-tcp-client.github
$ cd tokio-tcp-client
```

## Usage

```bash
$ RUST_LOG=debug cargo run
```

## Example output

```bash
$ RUST_LOG=debug cargo run
   Compiling tokio-tcp-client v0.1.0 (~/tokio-tcp-client)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/tokio-tcp-client`
2023-04-25T07:55:36.525004Z  INFO tokio_tcp_client: Stream incoming...
2023-04-25T07:55:36.525056Z DEBUG tokio_tcp_client: 101
```
