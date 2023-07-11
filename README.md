# LNBits Rust example

## Run
 
```bash
$ git clone https://github.com/dni/lnbits-client-rust-example.git
$ git submodule init
$ git submodule update --recursive
$ cargo run
   Compiling lnbits-example v0.1.0 (lnbits-example)
    Finished dev [unoptimized + debuginfo] target(s) in 2.11s
     Running `target/debug/lnbits-example`
create a tinyurl: Object {
    "endless": Bool(false),
    "id": String("cyVdQYfN"),
    "time": Number(1689087490.0),
    "url": String("https://600.wtf"),
    "wallet": String("61a61befc23c4cd7be111570a0a5cca7"),
}
get a tinyurl: Object {
    "endless": Bool(false),
    "id": String("cyVdQYfN"),
    "time": Number(1689087490.0),
    "url": String("https://600.wtf"),
    "wallet": String("61a61befc23c4cd7be111570a0a5cca7"),
}
delete a tinyurl: Object {
    "deleted": Bool(true),
}
```
