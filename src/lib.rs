use worker::*;

mod api;
use api::{security, data};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async(
            "/",
            |_, _| async move { Response::error("visit https://github.com/ArthurZhou/zline-api for details".to_string(), 404) },
        )
        .get_async("/security/xtoken", |_, _| async move {
            Response::from_json(&security::get_xtoken().await)
        })
        .get_async("/security/login", |req, _| async move {
            Response::from_json(&security::login(req).await)
        })
        .get_async("/security/logout", |_, _| async move {
            Response::from_json(&security::logout().await)
        })
        .get_async("/security/status", |req, _| async move {
            Response::from_json(&security::status(req).await)
        })
        .get_async("/data/exam_list", |req, _| async move {
            Response::from_json(&data::exam_list(req).await)
        })
        .run(req, env)
        .await
}
