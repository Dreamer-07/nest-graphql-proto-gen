pub mod format_typescript {
    use tera::{Error};

    use crate::core::parse_protobuf::parse_protobuf::ProtoBuf;

    pub fn fmt_ts_str(proto_buf: ProtoBuf) -> tera::Result<String> {
        let tera = match tera::Tera::new("template/*.html") {
            Ok(t) => t,
            Err(e) => return Err(Error::msg(format!("read template file error: {}", e)))
        };

        let enum_values: Vec<_> = proto_buf.enums.values().collect();
        let message_values: Vec<_> = proto_buf.messages.values().collect();
        let mut context = tera::Context::new();
        context.insert("enums", &enum_values);
        context.insert("messages", &message_values);

        tera.render("fmt_typescript.html", &context)
    }
}