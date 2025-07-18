mod get;
mod post;

pub use get::get_newsletter_form;
pub use post::{
    ERROR_MESSAGE, SUCCESS_MESSAGE, publish_newsletter,
};
