use std::collections::HashMap;

pub trait DeserializeJson {
    fn from_json(json: &HashMap<String, String>) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use deserialize_derive::DeserializeJson;

    #[derive(DeserializeJson, Default, Debug)]
    struct User {
        #[json_value(name = "age")]
        age: i64,
        #[json_value(name = "name")]
        name: Option<String>,
    }

    #[test]
    fn from_json() {
        let mut map = HashMap::<String, String>::new();
        map.insert("age".into(), "100".into());
        map.insert("name".into(), "test".into());
        let user = User::from_json(&map);
        assert_eq!(user.age, 100);
        assert_eq!(user.name, Some("test".into()));
    }
}
