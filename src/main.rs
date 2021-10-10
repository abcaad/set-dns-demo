extern crate base64;
extern crate crypto2;
use std::{fs::File, io::Read};
use {toml,reqwest};
use serde_derive::Deserialize;
use crypto2::blockcipher::Aes128;
use serde_json::Value;
use std::{collections::HashMap, env};

#[derive(Debug,Deserialize)]
struct Config {
    key: [u8;16],
    cf: Cf,
}
#[derive(Debug,Deserialize)]
struct Cf {
    url: String,
    authorization:String,
    i_type: String,
    i_name:String,
    i_content:String,
    i_ttl:String,
}


async fn set_record(cf:Cf) -> Result<Value, reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("type", cf.i_type);
    map.insert("name", cf.i_name);
    map.insert("content", cf.i_content);
    map.insert("ttl", cf.i_ttl);
    let client = reqwest::Client::new();
    Ok(client
        .put(cf.url)
        .header("Authorization", cf.authorization)
        .header("Content-Type", "application/dns-json")
        .json(&map)
        .send()
        .await?
        .json::<Value>()
        .await?
        // ["success"]
        // .as_bool()
        // .unwrap()
    )
}
fn get_encode_data(key:&mut [u8;16],raw:&str) -> String {
    let mut text = raw.as_bytes().to_owned();
    while text.len() % 16 != 0 {
        text.push(32);
    }
    let cipher = Aes128::new(key);
    cipher.decrypt(key);
    let cipher = Aes128::new(key);
    for i in (0..text.len()).step_by(16) {
        cipher.encrypt(text[i..i+16].as_mut());
    }
    base64::encode(text).replace("=", "")
}

#[tokio::main]
async fn main() {
    let args = env::args().nth(1).unwrap_or("".to_string());
    let mut file = File::open("config.toml").unwrap();
    let mut config_string = String::new();
    file.read_to_string(&mut config_string).unwrap();
    let mut config: Config = toml::from_str(&config_string).unwrap();

    config.cf.i_content=get_encode_data(&mut config.key,&args);
    println!("[{}]",&args);
    if let Ok(data) = set_record(config.cf).await {
        print!("{:#?}", data);
    }
}


#[cfg(test)]
mod tests {
    // use std::fmt::Write;
    use crypto2::blockcipher::Aes128;


    // use crate::set_record;
    #[test]
    fn it_works() {
        // println!("{}",get_encode_data());
    }
    #[test]
    fn split_array() {
        let mut ciphertext: Vec<u8> = vec![

        ];
        let mut plaintext = String::new();
        let mut key = [
            
        ];
        let cipher = Aes128::new(&key);
        cipher.decrypt(&mut key);
        let cipher = Aes128::new(&key);
        while ciphertext.len() > 15 {
            let decryptotext = ciphertext[0..16].as_mut();
            cipher.decrypt(decryptotext);
            // if let Ok(a) =  std::str::from_utf8_mut(decryptotext) {
            //     plaintext.push_str(a);
            // }
            plaintext.push_str(std::str::from_utf8(decryptotext).unwrap());
            // for a in decryptotext.iter() {
            //     // plaintext.push(a);

            //     write!(plaintext,"{:02x}",a);
            // }
            ciphertext = ciphertext[16..].to_vec();
        }
        println!("{}", plaintext);
    }
}
