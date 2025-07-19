use crate::{email_client::SendEmailRequest, hkt::RefHKT};

const _: () = {
    #[allow(
        unused_extern_crates,
        clippy::useless_attribute
    )]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<P: RefHKT> _serde::Serialize for SendEmailRequest<P> {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state =
                _serde::Serializer::serialize_struct(
                    __serializer,
                    "SendEmailRequest",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "From",
                &self.from,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "To",
                &self.to,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "Subject",
                &self.subject,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "HtmlBody",
                &self.html_body,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "TextBody",
                &self.text_body,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
