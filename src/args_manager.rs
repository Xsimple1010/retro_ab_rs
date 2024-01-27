use std::collections::HashMap;

fn get_key_and_value<'a>(arg: &'a str) -> (String, String) {
    let (mut key, mut value) = (String::from(""), String::from(""));

    if arg.starts_with("--") {
        let key_and_value = arg.replace("--", "");

        let k_v: Vec<&str> = key_and_value.rsplit("=").collect();

        key = k_v[1].to_string();

        value = k_v[0].to_string();
    }

    (key, value)
}

pub fn get_values(args: Vec<String>) -> HashMap<String, String> {
    let mut values: HashMap<String, String> = HashMap::new();

    for arg in args {
        if arg.contains(&"--core=") {
            let (key, value) = get_key_and_value(&arg);

            if !key.is_empty() {
                values.insert(key, value);
            }
        }

        if arg.contains(&"--rom=") {
            let (key, value) = get_key_and_value(&arg);

            if !key.is_empty() {
                values.insert(key, value);
            }
        }
    }

    values
}

#[test]
fn teste_get_values() {
    let mut args: Vec<String> = Vec::new();

    args.push("--core=test.c".to_string());
    args.push("--rom=test.r".to_string());

    let values = get_values(args);

    match values.get_key_value("rom") {
        Some((key, value)) => {
            assert_eq!(key, "rom");
            assert_eq!(value, "test.r");
        }
        None => panic!("o parâmetro 'rom' nao foi encontrado"),
    }

    match values.get_key_value("core") {
        Some((key, value)) => {
            assert_eq!(
                key, "core",
                "valor esperado para 'key' == 'core', valor encontrado -> {:?}",
                key
            );
            assert_eq!(
                value, "test.c",
                "valor esperado para 'value' == 'test.c', valor encontrado -> {:?}",
                value
            );
        }
        None => panic!("o parâmetro 'core' nao foi encontrado"),
    }
}

#[test]
fn test_get_key_and_value() {
    let command = "--core=teste.d";

    assert_eq!(
        get_key_and_value(command),
        ("core".to_string(), "teste.d".to_string())
    );
}
