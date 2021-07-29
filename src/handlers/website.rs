use actix_web::{get, HttpResponse, Responder};

use std::process::Command;



fn render_js() -> String {
    /*
    let output = Command::new("d8")
        .arg("ui/app.js")
        .output()
        .expect("d8 command failed to start");
    */
    let output = Command::new("echo")
        .arg("pending")
        .output()
        .expect("echo command failed to start");

    String::from_utf8_lossy(&output.stdout).into_owned()
}


// TODO
#[get("/")]
async fn handler_website() -> impl Responder {
    HttpResponse::Ok().body(render_js())
}

