#![allow(dead_code, unused_variables, unused_imports)]

use std::env;
use std::env::args;
use std::error::Error;
use std::fmt::format;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::string::FromUtf8Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use dotenv::dotenv;
use libc::{connect, time, time_t};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    index: i32,
    message: Message,
    finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

async fn ia_consult(query: String, format: String) -> Result<String, Box<dyn Error>> {
    let url = "https://api.openai.com/v1/chat/completions";
    let token = env::var("OPENAI_API_KEY")?;
    println!("TOKEN {:?}", token);
    let str = format!("write code of {query}, in {format} extension format");
    println!("{str}");
    let json = json!({
      "model": "gpt-4",
      "messages": [
        {"role": "system", "content": "Your response must be only code without delimiters. If cannot provide correct code you must response empty string."},
        {"role": "user", "content": str}
       ],
      "max_tokens":5000,
        "temperature":0.5
    });
    let client: reqwest::Response = Client::new()
        .post(url)
        .bearer_auth(token)
        .json(&json)
        .send()
        .await?;
    let body = client.text().await?;
    Ok(body)
}

async fn execute(code: String) -> Result<String, FromUtf8Error> {
    let cmd = Command::new("node").arg("-e").arg(code).output().unwrap();
    println!("node response -> {:?}", String::from_utf8(cmd.clone().stdout));

    return String::from_utf8(cmd.stdout);
}

async fn save_file(path_file: String, content: &[u8], file_type: String) -> Result<(), Box<dyn Error>> {
    let new_path = format!("./{}_{:?}.{}", path_file, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(), file_type);

    println!("Writing file {}...", new_path);

    let mut file = File::create(new_path).await.unwrap();
    file.write_all(content).await.expect("error cannot write content to file");
    Ok(())
}

async fn log(payload: String) {
    println!("Response log: {:?}", payload);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("PromptResultLogger.log")
        .expect("Error to open file log");
    file.write_all(payload.as_bytes()).expect("Cannot write content to file log");
    ()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let args: Vec<String> = args().collect();

    println!("{:?}", &args[1]);
    let query: &String = &args[1];
    let extension: &String = &args[2];

    if query.is_empty() { panic!("Ingresar el contenido a buscar") }
    if extension.is_empty() { panic!("Ingresar la extension para el archivo de salida") }


    let now=SystemTime::now();
    let body = ia_consult(String::from(query), extension.to_string()).await?;
    let duration=now.elapsed()?;
    log(body.clone()).await;
    let obj: Response = serde_json::from_str(body.as_str()).expect("Cannot convert string to json");
    let code = &obj.choices[0].message.content;

    // let str_code = serde_json::from_str(code)?;
    save_file(String::from("test"), code.as_bytes(), extension.to_string()).await?;

    println!("Duration time: {} seconds",  duration.as_secs());
    // let result=execute(code.clone());

    Ok(())
}
