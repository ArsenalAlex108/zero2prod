mod macros;
mod new_subsriber;
mod subscriber_email;
mod subscriber_name;

pub use new_subsriber::{
    NewSubscriber, NewSubscriberParseError,
};
pub use subscriber_email::{
    SubscriberEmail, SubscriberEmailParseError,
};
pub use subscriber_name::{
    SubscriberName, SubscriberNameParseError,
};
