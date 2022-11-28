#[macro_use] 
extern crate rocket;
extern crate serde;
use std::env;

use postgrest::Postgrest;
use rocket::{serde::{Serialize, Deserialize}};
use rocket::form::Form;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", None::<String>)
}

#[derive(FromForm, Serialize, Deserialize)]
struct Registrant<'r> {
    name: &'r str
}

#[derive(Serialize, Deserialize)]
struct SupabaseResp {
    id: u32,
    name: String,
}

#[post("/", data = "<registrant>")]
async fn post_index(registrant: Form<Registrant<'_>>) -> Template {

    let registrant = registrant.into_inner();
    // TODO this is hella messy 
    let client = Postgrest::new(env::var("supabase_rest_url").unwrap())
        .insert_header("apikey", env::var("supabase_api_key").unwrap());

    if let Ok(json) = serde_json::to_string(&registrant) {
        let ins = client.from(env::var("supabase_table_name").unwrap()).insert(json).execute().await.unwrap().text().await.unwrap();
        let sr: Vec<SupabaseResp> = serde_json::from_str(ins.as_str()).unwrap();
        let resp = sr.first().unwrap();
        Template::render("index", context! { registered: "true", number: resp.id.to_string().as_str(), name: resp.name.as_str() })
    } else {
        Template::render("index", context! { error: "Something went wrong. Let Brad know and he'll fix it!" })
    }

}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Template::fairing())
    .mount("/", routes![index, post_index])
}