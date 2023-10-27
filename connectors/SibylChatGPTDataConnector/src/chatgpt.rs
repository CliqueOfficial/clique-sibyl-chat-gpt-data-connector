use std::prelude::v1::*;
use sibyl_base_data_connector::utils::simple_tls_client;
use sibyl_base_data_connector::base::DataConnector;
use sibyl_base_data_connector::serde_json::Value;
use sibyl_base_data_connector::serde_json::json;
use sibyl_base_data_connector::errors::NetworkError;

use crate::env;

pub struct ChatGPTConnector {

}

impl DataConnector for ChatGPTConnector {
    fn query(&self, query_type: &Value, query_param: &Value) -> Result<Value, NetworkError> {
        let query_type_str = match query_type.as_str() {
            Some(r) => r,
            _ => {
                let err = format!("query_type to str failed");
                println!("{:?}", err);
                return Err(NetworkError::String(err));
            }
        };
        match query_type_str {
            "chatgpt_chat" => {
                let encoded_json = json!({
                    "model": "gpt-3.5-turbo-0301",
                    "messages": [
                        {
                            "role": "user",
                            "content": query_param["chat"]
                        }
                    ],
                    "temperature": 0.7
                }).to_string();
                let req = format!(
                    "POST {} HTTP/1.1\r\n\
                    HOST: {}\r\n\
                    User-Agent: curl/7.79.1\r\n\
                    Authorization: Bearer {}\r\n\
                    Accept: */*\r\n\
                    Content-Type: application/json\r\n\
                    Content-Length: {}\r\n\r\n\
                    {}",
                    env::CHATGPT_CHAT_SUFFIX,
                    env::CHATGPT_API_HOST,
                    query_param["bearer"].as_str().unwrap_or(""),
                    encoded_json.len(),
                    encoded_json
                );
                simple_tls_client(env::CHATGPT_API_HOST, &req, 443)
            },
            _ => {  
                Err(NetworkError::String(format!("Unexpected query_type: {:?}", query_type)))
            }
        }
    }
}

