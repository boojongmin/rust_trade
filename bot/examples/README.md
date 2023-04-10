# server에서 끊김을 클라이언트에서 체크를 바로 할 수 있는지 체크용. 

```bash
cargo run -p bot --example ws_server 
cargo run -p bot --example ws_client
```

server가 끊기자 for_each에서 바로 panic 발생

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Protocol(ResetWithoutClosingHandshake)', bot/examples/ws_client.rs:33:32
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

```rust
 read.for_each(|message| async {
    let data = message.unwrap().into_data();
    tokio::io::stdout().write_all(&data).await.unwrap();
})
```