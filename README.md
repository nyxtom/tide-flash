# tide-flash

[http-rs/tide](https://github.com/http-rs/tide) flash messages middleware with support for configurable flash storage through *Cookies*. Extensible to allow other storage mechanisms such as sessions. Primarily a mechanism for sending one time log messages to the client that may not maintain the state on redirects (such as on request logins). This is commonly used for authentication requests where the login is on an **HTTP POST** and a response will need to both: *return a redirect* as well as *one time log messages* to render.

## Examples

```rust
```
