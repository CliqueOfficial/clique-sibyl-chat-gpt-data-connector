use std::prelude::v1::*;
use sibyl_base_data_connector::base::DataConnector;
use sibyl_base_data_connector::serde_json::json;
use std::string::ToString;
use sibyl_base_data_connector::serde_json::Value;
use std::str;
use String;
use std::panic;
// use std::untrusted::time::SystemTimeEx;
use sibyl_base_data_connector::utils::{parse_result, tls_post};
use sibyl_base_data_connector::utils::simple_tls_client;

// ChatGPT GraphQL API
const CHATGPT_API_HOST: &'static str = "api.openai.com";
const CHATGPT_CHAT_SUFFIX: &'static str = "/v1/chat/completions";

pub struct ChatgptConnector {

}

impl DataConnector for ChatgptConnector {
    fn query(&self, query_type: &Value, query_param: &Value) -> Result<Value, String> {
        let query_type_str = match query_type.as_str() {
            Some(r) => r,
            _ => {
                let err = format!("query_type to str failed");
                println!("{:?}", err);
                return Err(err);
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
                    CHATGPT_CHAT_SUFFIX,
                    CHATGPT_API_HOST,
                    query_param["bearer"].as_str().unwrap_or(""),
                    encoded_json.len(),
                    encoded_json
                );

                let plaintext = match tls_post(CHATGPT_API_HOST, &req, 443) {
                    Ok(r) => r,
                    Err(e) => {
                        let err = format!("tls_post to str: {:?}", e);
                        println!("{:?}", err);
                        return Err(err);
                    }
                };
                let mut reason = "".to_string();
                let mut result: Value = json!("fail");
                match parse_result(&plaintext) {
                    Ok(r) => {
                        result = r;
                    },
                    Err(e) => {
                        reason = e;
                    }
                }
                // println!("parse result {:?}", result);
                Ok(json!({
                    "result": result,
                    "reason": reason
                }))
            },
            _ => {
                Err(format!("Unexpected query_type: {:?}", query_type))
            }
        }
    }
}

