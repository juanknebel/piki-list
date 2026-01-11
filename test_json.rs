use serde_json;

fn main() {
    let input = "[{a:1,b:2},{a:3,b:5}]";
    match serde_json::from_str::<serde_json::Value>(input) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }

    let input_valid = "[{\"a\":1,\"b\":2},{\"a\":3,\"b\":5}]";
    match serde_json::from_str::<serde_json::Value>(input_valid) {
        Ok(v) => {
            println!("Valid JSON Success: {}", v);
        }
        Err(e) => println!("Error: {}", e),
    }
}
