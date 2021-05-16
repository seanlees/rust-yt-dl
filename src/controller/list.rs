


use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;

use crate::{DbConn};
use crate::dict::LIST_TYPE_VIDEO;
use crate::models::DlInfo;
use crate::request::authenticated_user::AuthenticatedUser;

#[get("/list?<typ>")]
pub fn list(_user: AuthenticatedUser,
            conn: DbConn,
            typ: Option<String>) -> Template {
    let ttype = &typ.unwrap_or(LIST_TYPE_VIDEO.to_owned());

    //let dlInfoList = DlInfo::all(&conn);

    let dl_info_list = DlInfo::toggle_with_type(ttype, &conn);

    let mut context = Context::new();
    context.insert("dl_info_list", &dl_info_list);

    if &ttype == &LIST_TYPE_VIDEO {
        Template::render("list", &context)
    } else {
        Template::render("list_a", &context)
    }
}
