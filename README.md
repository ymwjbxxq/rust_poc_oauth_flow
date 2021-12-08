# Build a native module for NodeJs with Rust

## What is Neon

[Neon](https://neon-bindings.com/) is a library and toolchain for embedding Rust in your Node.js apps and libraries. Neon allows you to write type-safe and memory-safe, crash-free native Node modules, guaranteeing Rust’s parallelism with thread safety.

Neon has a few types https://neon-bindings.com/docs/primitive-types:

* JsValue
* subject
* And primitives like JsNumber

The key is to map your JS input to Rust and return an output mapping Rust to JS.


## How

The basic hello world is like this
```Rust
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}
```
And you can call from JS point to the export_function key.

```node
node
> require('.').hello()
hello node
```

While you will find just hello world or Fibonacci examples, I tried to do something more tangible. I tried to implement an HTTP client with [reqwest](https://github.com/seanmonstar/reqwest)

## How to Run

Change the URL that you want to HIT inside index.ts

1. npm install
2. build the native module under native folder running cargo build --release
3. from the root compile TypeScript with tsc
4. node index.js

You should be able to see something like this.

```bash
START native 2021-12-08T15:37:13.670Z
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
END*********1543ms
START node-fetch 2021-12-08T15:37:15.213Z
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
{ message: 'Go ahead without me' }
END*********138ms
```

As you can see, Rust took 1543ms vs pure Node 138ms

I think I have a problem with this code [Bridging with sync code](https://tokio.rs/tokio/topics/bridging)

```Rust
let http_response = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let res = fetch_url(&shared_request.url).await;
            res
        });
```

I am still working on this.
