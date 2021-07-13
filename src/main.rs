extern crate dmt_server;

#[rocket::main]
async fn main() {
    dmt_server::rocket_run().launch().await.unwrap();
}


