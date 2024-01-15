use std::collections::HashMap;

fn get_key_and_value<'a>(arg:&'a str) -> (String, String) {
    let mut values = (String::from(""), String::from(""));


    if arg.starts_with("--") {
        let key_and_value = arg.replace("--", "");
    
        let k_v:Vec<&str> = key_and_value.rsplit("=").collect();
        
        values.0 = k_v[1].to_string();
        
        values.1 = k_v[0].to_string();
    }



    values
}

pub fn get_values(args:&Vec<String>) -> HashMap<String, String> {
    let mut values: HashMap<String, String> = HashMap::new();

    for arg in args {
        if arg.contains(&"--core=") {
            let value = get_key_and_value(arg);

            if !value.0.is_empty() {
                values.insert(value.0, value.1);
            }
        }

        if arg.contains(&"--rom=") {
            let value = get_key_and_value(arg);

            if !value.0.is_empty() {
                values.insert(value.0, value.1);
            }
        }
    }

    return values;
}