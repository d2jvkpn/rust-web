use crate::{
    internal::settings::get_chatgpt,
    middlewares::{Error, IntoResult},
    models::chat::Message,
    models::chatgpt::ChatCompletionsResponse,
};
use actix_web::{error::Error as ActixError, web, HttpRequest, HttpResponse};

pub async fn handle_msg(
    mut request: HttpRequest,
    msg: web::Json<Message>,
) -> Result<HttpResponse, ActixError> {
    // Ok(HttpResponse::Ok(Message::new("HELLO".into())).json())

    handle(msg.into_inner()).await.into_result(&mut request)
}

async fn handle(msg: Message) -> Result<ChatCompletionsResponse, Error> {
    // println!("~~~ got message: {:?}", msg);
    // let ans = Message::new("HELLO".into());

    let client = get_chatgpt().ok_or(Error::unexpected_error())?;
    let ccr = msg.into();

    // TODO: handler error
    client.chat_completions(&ccr).await.map_err(|_| Error::unexpected_error())
}
