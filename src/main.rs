extern crate dmt_server;

#[rocket::main]
async fn main() {
    match dmt_server::rocket_run().launch().await {
        Ok(_) => print!("Ran successfully"),
        Err(value) => print!("Error: {}", value)
    };
}


