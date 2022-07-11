# tide-flash

[http-rs/tide](https://github.com/http-rs/tide) flash messages middleware with support for configurable flash storage through *Cookies*. Extensible to allow other storage mechanisms such as sessions. Primarily a mechanism for sending one time log messages to the client that may not maintain the state on redirects (such as on request logins). This is commonly used for authentication requests where the login is on an **HTTP POST** and a response will need to both: *return a redirect* as well as *one time log messages* to render.

## Examples

To setup flash middleware with the configured *CookieStore* use the defaults below:

```rust
use tide::log::LogMiddleware;
use tide_flash::{cookies::CookieStore, FlashMiddleware};

mod routes;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    dotenv::dotenv().ok();
    env_logger::init();

    app.with(LogMiddleware::new());
    app.with(FlashMiddleware::new(CookieStore::default()));

    routes::configure(&mut app);

    let host = std::env::var("HOST").unwrap_or(String::from("0.0.0.0"));
    let port: u16 = std::env::var("PORT")?.parse()?;
    app.listen((host, port)).await?;

    Ok(())
}
```

Defaults for the CookieStore are:

```rust
impl Default for CookieConfig {
    fn default() -> Self {
        Self {
            max_age: time::Duration::seconds(60),
            site: SameSite::Lax,
            http_only: true,
            path: String::from("/"),
        }
    }
}
```
