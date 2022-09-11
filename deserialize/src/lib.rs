use std::collections::HashMap;

pub trait DeserializeJson {
    fn from_json(json: &HashMap<String, String>) -> Self;
}

pub trait ParseList<U> {
    type Err;
    fn parse_list(&self) -> Result<Vec<U>, Self::Err>;
}

impl ParseList<String> for String {
    type Err = String;

    fn parse_list(&self) -> Result<Vec<String>, Self::Err> {
        let index = self.find(',');
        let mut string_list = Vec::new();
        if index.is_none() {
            let list = self.split_whitespace();
            for x in list {
                string_list.push(String::from(x.trim()).clone());
            }
        } else {
            let list = self.split(',');
            for x in list {
                string_list.push(String::from(x.trim()).clone());
            }
        }
        Ok(string_list)
    }
}

#[macro_export]
macro_rules! from_str_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$value_meta:meta])* $value_name:ident,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$value_meta])* $value_name),*
        }

        impl FromStr for $name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($value_name) => Ok($name::$value_name),)*
                    _ => Err("Unknown".into()),
                }
            }
        }

    };
}

#[macro_export]
macro_rules! from_str_enum_value {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$value_meta:meta])* $value_name:ident $(= $value:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$value_meta])* $value_name $(= $value)?),*
        }

        impl FromStr for $name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($($value)+) => Ok($name::$value_name),)*
                    _ => Err("Unknown".into()),
                }
            }
        }

    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use deserialize_derive::DeserializeJson;
    use std::str::FromStr;

    from_str_enum! {
        #[derive(PartialEq, Debug)]
        enum Platform {
            Windows,
            Linux,
            Mac,
        }
    }

    from_str_enum_value! {
        #[derive(PartialEq, Debug)]
        enum PlatformValue {
            Windows = 0,
            Linux = 1,
            Mac = 2,
        }
    }

    #[derive(DeserializeJson, Default, Debug)]
    struct User {
        #[json_value(name = "age")]
        age: i64,
        #[json_value(name = "name")]
        name: Option<String>,
        #[json_value(name = "tags")]
        tags: Vec<String>,
    }

    #[test]
    fn from_json() {
        let mut map = HashMap::<String, String>::new();
        map.insert("age".into(), "100".into());
        map.insert("name".into(), "test".into());
        map.insert("tags".into(), "a b c de f".into());
        let user = User::from_json(&map);
        assert_eq!(user.age, 100);
        assert_eq!(user.name, Some("test".into()));
        assert_eq!(user.tags, vec!["a", "b", "c", "de", "f"]);
    }

    #[test]
    fn enum_form_str() {
        assert_eq!(Platform::from_str("Windows"), Ok(Platform::Windows));
    }

    #[test]
    fn enum_form_value() {
        assert_eq!(PlatformValue::from_str("1"), Ok(PlatformValue::Linux));
    }
}
