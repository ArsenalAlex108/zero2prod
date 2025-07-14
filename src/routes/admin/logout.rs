use crate::{
    authentication::UserId, session_state::TypedSession,
    utils::see_other_response,
};
use actix_web::{HttpResponse, web};

pub async fn logout(
    _user_id: web::ReqData<UserId>,
    session: TypedSession,
) -> HttpResponse {
    session.logout();
    actix_web_flash_messages::FlashMessage::info(
        "Logged out successfully.",
    )
    .send();
    see_other_response("/login")
}
