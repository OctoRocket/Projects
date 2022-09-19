#[macro_use] extern crate rocket;
use std::path::{Path, PathBuf};
use rocket::{fs::NamedFile, response::Redirect};


#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/index.html"))
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![files])
}
