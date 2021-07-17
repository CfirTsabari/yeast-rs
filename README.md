<h1 align="center">yeast-rs</h1>
<div align="center">
 <strong>
    Tiny but linear growing unique id generator.
    <br>
    Rust implementation of the JS package - <a href="https://github.com/unshiftio/yeast">yeast</a>
 </strong>
</div>

<div align="center">
</div>


## Usage

### Install

```toml
[dependencies]
yeast-rs = "0.1.0"
```

### Simple
This crate has one function `yeast` which return `Yeast` a URL-safe unique id.

```rust
use yeast_rs::yeast;

fn main() {
   let id = yeast().to_string(); //=> "NgqS4Rd"
}
```

### async support
Currently, we support `tokio` and `async-std`
```toml
[dependencies]
yeast-rs = {version ="0.1.0",features=["async-std-runtime"]}
```

```rust
use yeast_rs::async_std::yeast;
async fn func(){
    let id = yeast().await.to_string(); //=> "NgqS4Rd"
}
```
