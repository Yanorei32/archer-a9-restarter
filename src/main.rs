use std::env;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct StokData {
    stok: String,
}

#[derive(Deserialize, Debug)]
struct LoginResponse {
    success: bool,
    data: StokData,
}

#[derive(Deserialize, Debug)]
struct RebootData {
    reboot_time: isize,
}

#[derive(Deserialize, Debug)]
struct RebootResponse {
    success: bool,
    data: RebootData,
}

#[tokio::main]
async fn main() {
    let base_uri = format!("http://{}", env::var("ARCHERA9_HOST").unwrap());

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let params = [
        ("operation", "login"),
        ("password", &env::var("ARCHERA9_PASSWORD").unwrap()),
    ];

    let d: LoginResponse = client.post(&format!("{}/cgi-bin/luci/;stok=/login?form=login", &base_uri))
        .form(&params).send().await.unwrap().json().await.unwrap();

    println!("{:?}", d);

    if !d.success {
        panic!("failed to login")
    }

    let params = [
        ("operation", "write"),
    ];

    let d: RebootResponse = client
        .post(&format!("{}/cgi-bin/luci/;stok={}/admin/system?form=reboot_commit", &base_uri, &d.data.stok))
        .form(&params).send().await.unwrap().json().await.unwrap();

    println!("{:?}", d);

    if !d.success {
        panic!("failed to reboot")
    }
}

