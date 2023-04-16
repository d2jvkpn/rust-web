use crate::{
    middlewares::{Error, IntoResult},
    models::chat::Message,
};
use actix_web::{error::Error as ActixError, web, HttpRequest, HttpResponse};

pub async fn handle_msg(
    mut request: HttpRequest,
    msg: web::Json<Message>,
) -> Result<HttpResponse, ActixError> {
    // Ok(HttpResponse::Ok(Message::new("HELLO".into())).json())

    handle(msg.into_inner()).into_result(&mut request)
}

fn handle(msg: Message) -> Result<Message, Error> {
    println!("~~~ got message: {:?}", msg);
    let ans = Message::new("HELLO".into());

    Ok(ans)
}
