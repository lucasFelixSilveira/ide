use actix_web::{web, get, Responder, HttpResponse};

#[get("/")]
pub async fn root() -> impl Responder {
    String::from("Server is on.")
}

#[get("/shutdown/{coop_ready}")]
pub async fn shutdown(coop_ready: web::Path<String>) -> impl Responder {
    if format!("{}", coop_ready) == "true" {
        println!("Killed coop server.");
        std::thread::sleep(std::time::Duration::new(1, 0));
    }
    println!("Killed process.");

    std::process::exit(0);
    HttpResponse::Ok().finish()
}

#[get("/new_user/{username}")]
pub async fn new_user(username: web::Path<String>) -> impl Responder {
    std::fs::write("emited_event", "new_user").unwrap();
    std::fs::write("new_user", format!("{username}")).unwrap();

    String::new()
}

#[get("/buffer_content")]
pub async fn buffer_content() -> impl Responder {
    match std::fs::read_to_string(format!("{}{}.file_", std::env::current_exe().unwrap().display().to_string().rsplit_once(std::path::MAIN_SEPARATOR).unwrap().0.to_string(), std::path::MAIN_SEPARATOR)) {
        Ok(directory) => match std::fs::read_to_string(directory) {
            Ok(content) => content,
            Err(_) => String::from("Finding buffer...")
        },
        Err(_) => String::from("Finding buffer...")
    }
}

#[get("/values")]
pub async fn values() -> impl Responder {
    match 
        std::fs::read_to_string(
            format!(
                "{}{}.values_", 
                std::env::current_exe()
                    .unwrap()
                    .display()
                    .to_string()
                    .rsplit_once(std::path::MAIN_SEPARATOR)
                    .unwrap().0
                    .to_string(), 
                std::path::MAIN_SEPARATOR
            )
        )
    {
        Ok(content) => content,
        Err(_) => String::from("0 0 0 = Unknown")
    }
}