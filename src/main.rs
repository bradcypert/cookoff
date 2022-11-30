#[macro_use] 
extern crate rocket;
extern crate serde;
use std::env;

use postgrest::Postgrest;
use rocket::{serde::{Serialize, Deserialize}, State, http::Status};
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

struct TableNames {
    registrant: String
}

#[post("/", data = "<registrant>")]
async fn post_index(registrant: Form<Registrant<'_>>, client: &State<Postgrest>, tables: &State<TableNames>) -> Result<Template, Status> {
    let registrant = registrant.into_inner();

    if let Ok(json) = serde_json::to_string(&registrant) {
        let ins = client.from(&tables.registrant).insert(json).execute().await;
        match ins {
            Ok(response) =>  match response.text().await.ok() {
                Some(body) => {
                    let items: Vec<SupabaseResp> = serde_json::from_str(body.as_str()).unwrap();
                    if let Some(resp) = items.first() {
                        Ok(Template::render("index", context! { registered: "true", number: resp.id.to_string().as_str(), name: resp.name.as_str() }))
                    } else {
                        Err(Status::InternalServerError)
                    }
                },
                _ => {
                    // unable to get text from response body
                    Err(Status::InternalServerError)
                },
            },
            Err(_) => {
                Err(Status::FailedDependency)
            },
        }
    } else {
        Ok(Template::render("index", context! { error: "Something went wrong. Let Brad know and he'll fix it!" }))
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Template::fairing())
    .mount("/", routes![index, post_index])
    .manage(Postgrest::new(env::var("supabase_rest_url").unwrap())
        .insert_header("apikey", env::var("supabase_api_key").unwrap()))
    .manage(TableNames {
        registrant: env::var("supabase_table_name").unwrap()
    })
}