use rocket::http::{Cookies, Cookie};
use rocket::response::{Redirect, Flash};

/*pub fn login(mut cookies: Cookies) -> std::result::Result<Redirect, Flash<Redirect>> {
    //获取用户，可以通过创建一个LoginForm类型，并为之实现FromRequest trait，来传入登录信息
    let user = User {
        name: String::from("login user"),
    };
    cookies.add_private(Cookie::new("login user", user.name));
}*/
