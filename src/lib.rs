mod api;
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/", |_, _| async move {
            Response::error(
                "".to_string(),
                404,
            )
        })
        .post_async("/security/xtoken", |_, _| async move {
            Response::from_json(&api::get_xtoken().await)
        })
        .run(req, env)
        .await
}
