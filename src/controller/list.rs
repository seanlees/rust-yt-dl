use std::collections::HashMap;

use rocket_contrib::templates::{Template};

use crate::{dict, DbConn};
use crate::request::authenticated_user::AuthenticatedUser;
use crate::dict::listTypeVideo;
use rocket_contrib::databases::diesel::{sql_query, RunQueryDsl};
use crate::models::DlInfo;
use rocket_contrib::templates::tera::Context;


#[get("/list?<typ>")]
pub fn list(_user: AuthenticatedUser,
            conn: DbConn,
            typ: Option<String>) -> Template {
    let ttype = &typ.unwrap_or(listTypeVideo.to_owned());

    //let dl_list = sql_query("SELECT * FROM dl_list ORDER BY id")
    //    .load(&conn);

    let dlInfoList = DlInfo::all(&conn);

    let mut context = Context::new();
    context.insert("dlInfoList", &dlInfoList);

    if &ttype == &listTypeVideo {
        Template::render("list", &context)
    } else {
        Template::render("list_a", &context)
    }
}
