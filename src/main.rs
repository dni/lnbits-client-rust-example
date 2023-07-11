use openapi::apis::{configuration::{Configuration, ApiKey}, core_api::{tinyurl_api_v1_tinyurl_post, tinyurl_api_v1_tinyurl_tinyurl_id_get, tinyurl_api_v1_tinyurl_tinyurl_id_delete}};

#[tokio::main]
async fn main() {
    let mut conf = Configuration::new();
    conf.base_path = "https://legend.lnbits.com".into();
    conf.api_key = Option::Some(ApiKey{
        key: "15dbed9c69eb498a8bb7db6d2b527d59".into(),
        prefix: None,
    });

    let create = tinyurl_api_v1_tinyurl_post(&conf, "https://600.wtf", Some(true)).await.ok().unwrap();
    println!("create a tinyurl: {:#?}", create);
    
    let id = create.get("id").unwrap().as_str().unwrap();
    
    let get = tinyurl_api_v1_tinyurl_tinyurl_id_get(&conf, id).await.ok().unwrap();
    println!("get a tinyurl: {:#?}", get);

    let delete = tinyurl_api_v1_tinyurl_tinyurl_id_delete(&conf, id).await.ok().unwrap();
    println!("delete a tinyurl: {:#?}", delete);
}
