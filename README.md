# 云想时空核心包
## response utils 返回结果工具包
要返回统一结果，函数返回值定义为 impl IntoResponse
```rust
fn demo() -> impl IntoResponse {
    ResResponse::with_success("")
}
```
## cts-license
```shell
cargo install --path cts-license 
```