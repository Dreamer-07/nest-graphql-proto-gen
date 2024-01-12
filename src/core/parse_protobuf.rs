pub mod parse_protobuf {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io;
    use std::io::BufRead;
    use std::path::PathBuf;

    use regex::Regex;

    #[derive(Debug)]
    pub struct MessageField {
        name: String,
        field: String,
        optional: bool,
        repeated: bool,
    }

    impl MessageField {
        pub fn new(name: String, field: String, optional: bool, repeated: bool) -> MessageField {
            MessageField { name, field, optional, repeated }
        }
    }

    #[derive(Debug)]
    pub struct EnumField {
        name: String,
        value: u32,
    }

    impl EnumField {
        pub fn new(name: String, value: u32) -> EnumField {
            EnumField { name, value }
        }
    }

    #[derive(Debug)]
    pub struct ProtoBuf {
        messages: HashMap<String, Vec<MessageField>>,
        enums: HashMap<String, Vec<EnumField>>,
    }

    impl ProtoBuf {
        pub fn new() -> ProtoBuf {
            ProtoBuf {
                messages: HashMap::new(),
                enums: HashMap::new(),
            }
        }
    }

    fn parse_message(data: &str) -> Result<MessageField, String> {
        // 切割字符串
        let mut parts = data.split_whitespace().collect::<Vec<_>>();
        if parts.len() < 2 {
            return Err(format!("invalid data {}", data));
        }
        // 检查是否标有 repeated / optional 关键字
        let check = parts[0];
        let repeated = check == "repeated";
        let optional = check == "optional";
        // 如果含有 repeated / optional 关键字就表示第二个元素才是属性类型
        let field_type_idx = if repeated || optional {
            1
        } else {
            0
        };
        let field_type = parts[field_type_idx];
        let field_name = parts[field_type_idx + 1];

        Ok(MessageField::new(field_name.to_string(), field_type.to_string(), optional, repeated))
    }

    fn parse_enum(data: &str) -> Result<EnumField, String> {
        // 切割字符串
        let mut parts = data.split("=").collect::<Vec<_>>();
        if parts.len() < 1 {
            return Err(format!("invalid data {}", data));
        }

        let field_name = parts[0].trim();
        let field_value = parts[1].trim().replace(";", "");
        Ok(EnumField::new(field_name.to_string(), field_value.to_string().parse().unwrap()))
    }

    pub fn parse_proto(file_path: &PathBuf) -> io::Result<ProtoBuf> {
        let message_re: Regex = Regex::new(r"message (\w+)\s*\{").unwrap();
        let enum_re: Regex = Regex::new(r"enum (\w+)\s*\{").unwrap();
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut messages_stack: Vec<(String, u8)> = Vec::new();
        let mut proto_buf = ProtoBuf::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed_line = line.trim();

            if trimmed_line.starts_with("message") {
                let message_captures = message_re.captures(trimmed_line).unwrap().get(1);
                let message_name = message_captures.map_or("", |m| m.as_str());
                messages_stack.push((message_name.to_string(), 0));
                proto_buf.messages.insert(message_name.to_string(), Vec::new());
            } else if trimmed_line.starts_with("enum") {
                let enum_captures = enum_re.captures(trimmed_line).unwrap().get(1);
                let enum_name = enum_captures.map_or("", |m| m.as_str());
                messages_stack.push((enum_name.to_string(), 1));
                proto_buf.enums.insert(enum_name.to_string(), Vec::new());
            } else if trimmed_line.starts_with("}") {
                messages_stack.pop();
            } else if !messages_stack.is_empty() {
                let (data_name, data_type) = messages_stack.last().unwrap();
                if *data_type == 0 {
                    proto_buf.messages.get_mut(data_name).unwrap().push(parse_message(trimmed_line).unwrap());
                } else {
                    proto_buf.enums.get_mut(data_name).unwrap().push(parse_enum(trimmed_line).unwrap())
                }
            }
        }

        Ok(proto_buf)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::parse_protobuf::parse_protobuf;

    #[test]
    fn test_parse_proto() {
        let proto_path_buf: PathBuf = PathBuf::from("proto/article.proto");

        parse_protobuf::parse_proto(&proto_path_buf).expect("read file error");
    }
}