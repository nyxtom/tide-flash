# tide-flash

[![Latest version](https://img.shields.io/crates/v/tide-flash.svg)](https://crates.io/crates/tide-flash)
[![Documentation](https://docs.rs/tide-flash/badge.svg)](https://docs.rs/tide-flash)
![License](https://img.shields.io/crates/l/tide-flash.svg)

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

Now to simply use it you can import the response extensions through:

```rust
use tide_flash::ext::*;
use tide::{Response, Request, Redirect};

pub async fn authenticate(mut req: Request<State>) -> tide::Result {
    match req.body_form::<UserForm>().await {
        Ok(form) => {
            if form.username == "foo" && form.password == "bar" {
                let claims = Claims {
                    username: String::from("foo"),
                    exp: 10000000000,
                    sub: String::from("asdf"),
                    uid: 1,
                };
                req.login(claims)?;
                Ok(Redirect::new("/").into())
            } else {
                let mut res: Response = Redirect::new("/").into();
                res.flash_error("invalid credentials");
                Ok(res)
            }
        }
        Err(e) => {
            let mut res: Response = Redirect::new("/").into();
            res.flash_error(e.to_string());
            Ok(res)
        }
    }
}
```

There are a number of simple response extension utilities you can use here including:

```rust
pub trait ResponseFlashExt {
    fn flash(&mut self, level: &str, msg: String);
    fn flash_success<S: Into<String>>(&mut self, msg: S);
    fn flash_info<S: Into<String>>(&mut self, msg: S);
    fn flash_warn<S: Into<String>>(&mut self, msg: S);
    fn flash_error<S: Into<String>>(&mut self, msg: S);
    fn flash_debug<S: Into<String>>(&mut self, msg: S);
}
```

To obtain the corresponding flash messages from the `tide::Request` use the **flash** function on the request extension:

```rust
impl<'request, T: Serialize> Context<'request, T> {
    pub fn with_data(request: &'request Request<State>, data: T) -> Self {
        Self {
            request,
            flash: request.flash(),
            claims: request.claims(),
            data: Some(data),
        }
    }

    pub fn render(self, template: &str) -> Result<String, handlebars::RenderError> {
        self.request.state().render(
            template,
            &serde_json::json!({
                "flash": self.flash,
                "claims": self.claims,
                "data": self.data
            }),
        )
    }
}
```
