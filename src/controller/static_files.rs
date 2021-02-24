use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::Cursor;
use std::path::PathBuf;

use rocket::http::{ContentType, Status};
use rocket::response;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/static"]
struct Asset;

#[get("/static/<file..>")]
pub fn haddle<'r>(file: PathBuf) -> response::Result<'r> {
    let filename = file.display().to_string();
    Asset::get(&filename).map_or_else(
        || Err(Status::NotFound),
        |d| {
            let ext = file
                .as_path()
                .extension()
                .and_then(OsStr::to_str)
                .ok_or_else(|| Status::new(400, "Could not get file extension"))?;
            let content_type = ContentType::from_extension(ext).ok_or_else(|| Status::new(400, "Could not get file content type"))?;
            response::Response::build().header(content_type).sized_body(Cursor::new(d)).ok()
        },
    )
}