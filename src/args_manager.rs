pub struct ArgValue {
    key: String,
    value: String,
}

fn get_key_and_value<'a>(arg:&'a str) -> ArgValue {
    let mut values: ArgValue = ArgValue { 
        key: (String::from("")), 
        value: (String::from("")) 
    };


    if arg.starts_with("--") {
        let key_and_value = arg.replace("--", "");
    
        let k:Vec<&str> = key_and_value.rsplit("=").collect();
        
        values.key = k[0].to_string();
        values.value = k[1].to_string();
    }



    values
}

pub fn get_values(args:&Vec<String>) -> Vec<ArgValue> {
    let mut values: Vec<ArgValue> = Vec::new();

    for arg in args {
        if arg.contains(&"--core=") {
            let value = get_key_and_value(arg);

            if !value.key.is_empty() {
                values.push(value);
            }
        }

        if arg.contains(&"--rom=") {
            let value = get_key_and_value(arg);

            if !value.key.is_empty() {
                values.push(value);
            }
        }
    }

    return values;
}