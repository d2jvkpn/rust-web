use crate::{
    internal::settings::get_chatgpt,
    middlewares::{Error, IntoResult},
    models::chat::Message,
    models::chatgpt::ChatCompletionsRequest,
};
use actix_web::{error::Error as ActixError, web, HttpRequest, HttpResponse};

pub async fn chat_completions(
    mut request: HttpRequest,
    msg: web::Json<Message>,
) -> Result<HttpResponse, ActixError> {
    // Ok(HttpResponse::Ok(Message::new("HELLO".into())).json())

    let client = get_chatgpt().ok_or(Error::unexpected_error())?;
    let ccr: ChatCompletionsRequest = msg.into_inner().into();

    client
        .chat_completions(&ccr)
        .await
        .map_err(|_| Error::unexpected_error())
        .into_result(&mut request)
}
