#[cfg(test)]
mod tests {
    use crate::{amf0, amf3};

    use std::collections::HashMap;

    macro_rules! amf0_test {
        ($expected:expr) => {
            let mut encoded = Vec::<u8>::new();
            amf0::encoder::to_bytes(&mut encoded, &$expected).unwrap();
            let decoded = amf0::decoder::from_bytes(&mut encoded.as_slice()).unwrap();
            assert_eq!(decoded, $expected)
        };
    }

    #[test]
    #[cfg(feature = "amf0-boolean")]
    fn amf0_boolean() {
        amf0_test!(amf0::Value::Boolean(true));
        amf0_test!(amf0::Value::Boolean(false));
    }

    #[test]
    fn amf0_null() {
        amf0_test!(amf0::Value::Null);
    }

    #[test]
    fn amf0_undefined() {
        amf0_test!(amf0::Value::Undefined);
    }

    #[test]
    fn amf0_number() {
        amf0_test!(amf0::Value::Number(3.5));
        amf0_test!(amf0::Value::Number(f64::INFINITY));
        amf0_test!(amf0::Value::Number(f64::NEG_INFINITY));
    }

    #[test]
    fn amf0_string() {
        amf0_test!(amf0::Value::String("this is test".to_string()));
        amf0_test!(amf0::Value::String("this is 테스트".to_string()));
        amf0_test!(amf0::Value::String("this is テスト".to_string()));
    }

    #[test]
    fn amf0_long_string() {
        amf0_test!(amf0::Value::String(
            std::iter::repeat('a').take(0x10013).collect()
        ));
    }

    #[test]
    fn amf0_object() {
        amf0_test!(amf0::Value::Object(crate::Object {
            class_name: "".to_string(),
            property: HashMap::from([
                ("".to_string(), amf0::Value::String("".to_string())),
                ("foo".to_string(), amf0::Value::String("baz".to_string())),
                ("bar".to_string(), amf0::Value::Number(3.14)),
            ]),
        }));
        amf0_test!(amf0::Value::Object(crate::Object {
            class_name: "".to_string(),
            property: HashMap::from([
                ("foo".to_string(), amf0::Value::String("bar".to_string())),
                ("baz".to_string(), amf0::Value::Null),
            ]),
        }));
    }

    #[test]
    fn amf0_typed_object() {
        amf0_test!(amf0::Value::Object(crate::Object {
            class_name: "org.amf.ASClass".to_string(),
            property: HashMap::from([
                ("foo".to_string(), amf0::Value::String("bar".to_string())),
                ("baz".to_string(), amf0::Value::Null),
            ]),
        }));
    }

    #[test]
    fn amf0_ecma_array() {
        amf0_test!(amf0::Value::ECMAArray(HashMap::from([
            ("0".to_string(), amf0::Value::String("a".to_string())),
            ("1".to_string(), amf0::Value::String("b".to_string())),
            ("2".to_string(), amf0::Value::String("c".to_string())),
            ("3".to_string(), amf0::Value::String("d".to_string())),
        ])));
    }

    #[test]
    fn amf0_strict_array() {
        amf0_test!(amf0::Value::StrictArray(vec![
            amf0::Value::Number(1.0),
            amf0::Value::String("2".to_string()),
            amf0::Value::Number(3.0),
        ]));
    }

    #[test]
    fn amf0_date() {
        amf0_test!(amf0::Value::Date(crate::Date::new(1_590_796_800_000_f64)));
        amf0_test!(amf0::Value::Date(crate::Date::new(1_045_112_400_000_f64)));
        amf0_test!(amf0::Value::Date(crate::Date::new(-1.0)));
        amf0_test!(amf0::Value::Date(crate::Date::new(f64::INFINITY)));
    }

    #[test]
    fn amf0_xml_doc() {
        amf0_test!(amf0::Value::String(
            "<parent><child prop=\"test\" /></parent>".to_string()
        ));
    }
}
