use std::collections::HashMap;
use std::path::{Path, PathBuf};
use rocket::request::Request;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use controller::user::{UserId,UserOr};
use handler::content::{Uarticle,article_list,get_unread_message_count};
use model::pg::ConnPg;

#[derive(Serialize)]
struct TemplateContext {
    datas: Vec<Uarticle>,
    username: String,
    user_id: i32,
    count: i32,
}
#[derive(Serialize)]
struct TemplateDoc {
    username: String,
    user_id: i32,
}


#[get("/",rank = 2)]
pub fn index(conn_pg: ConnPg) -> Template {
    let datas = article_list(&conn_pg);
    let context = TemplateContext {
        datas: datas,
        username: "".to_string(),
        user_id: 0,
        count: 0,
    };
    Template::render("index", &context)
}

#[get("/")]
pub fn index_user(conn_pg: ConnPg, user: UserOr, user_id: UserId) -> Template {
    let datas = article_list(&conn_pg);
    let count = get_unread_message_count(&conn_pg, user_id.0);
    let context = TemplateContext {
        datas: datas,
        username: user.0,
        user_id: user_id.0,
        count:count,
    };
     
    Template::render("index", &context)
}

#[get("/wiki",rank = 2)]
pub fn wiki() -> Template {
    let mut context = HashMap::new();
    context.insert("No login user", "".to_string());
    Template::render("wiki", &context)
}

#[get("/wiki")]
pub fn wiki_user(user: UserOr, user_id: UserId) -> Template {
    let context = TemplateDoc {
        username: user.0,
        user_id: user_id.0,
    };
    Template::render("wiki", &context)
}




#[get("/<file..>",rank = 9)]
pub fn public(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

#[error(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}
