use gstd::format;
use gstd::scale_info::TypeDef;
use gstd::String;
use gstd::ToString;
use gstd::TypeInfo;
use gstd::Vec;

pub fn parse_type<Value>() -> (Vec<String>, Vec<String>)
where
    Value: TypeInfo,
{
    let mut key_names: Vec<String> = Vec::new();
    let mut key_types: Vec<String> = Vec::new();

    let type_info = Value::type_info();

    if type_info.path.is_empty() {
        key_names.push("value".into());
        let ty = match type_info.type_def {
            TypeDef::Primitive(primitive) => {
                format!("{:?}", primitive)
            }
            _ => "".into(),
        };
        key_types.push(ty);
    } else {
        match type_info.type_def {
            TypeDef::Composite(composite) => {
                if type_info.path.segments.last() == Some(&"ActorId") {
                    key_names.push("value".into());
                    key_types.push("ActorId".into());
                } else {
                    composite
                        .fields
                        .iter()
                        .map(|field| {
                            key_names.push(field.name.unwrap().to_string());
                            key_types.push(field.type_name.unwrap().to_string());
                        })
                        .collect()
                }
            }
            _ => {}
        }
    }
    (key_names, key_types)
}
