use actix_web::{get, App, HttpResponse, HttpServer, Responder, Error, web, HttpRequest};
use serde::{Deserialize, Serialize};
use mongodb::{Client, Collection};

#[derive(Deserialize, Serialize)]
struct IndexQuery {
    hello: String
}

#[get("/test1")]
async fn test_1(
    req: HttpRequest,
    client: web::Data<Client>
) -> Result<impl Responder, Error> {
    let params = web::Query::<IndexQuery>::from_query(req.query_string()).unwrap();

    let col: Collection<User> = client.database("foo").collection("bar");
    let doc_json = format!(r#"
        {{"$where": "this.name == '{}'"}}
    "#, params.hello);

    let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

    // ruleid: AIK_Rust-nosql-find-injection
    let result = col.find(filter_doc, None).await.unwrap();

    let raw_doc = result.current();
    do_smth("{:?}", raw_doc );

    Ok(HttpResponse::Ok().body("Hello world!!!"))
}

#[post("/test2")]
async fn test_2(
  info: web::Path<Info>,
  client: web::Data<Client>,
) -> Result<String> {
  let col: Collection<User> = client.database("foo").collection("bar");
  let doc_json = "{{\"$where\": \"this.name == '" + String::from(info.username) + "'\"}}";

  let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

  // ruleid: AIK_Rust-nosql-find-injection
  let result = col.find_one(filter_doc, None).await.unwrap();
  do_smth(result);

  Ok("Welcome!")
}

#[post("/test3")]
async fn test_3(
  info: web::Path<Info>,
  col: web::Data<Collection<User>>,
) -> Result<String> {
  let doc_json = "{{\"$where\": \"this.name == '" + String::from(info.username) + "'\"}}";

  let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

  // ruleid: AIK_Rust-nosql-find-injection
  let result = col.find_one(filter_doc, None).await.unwrap();
  do_smth(result);

  Ok("Welcome!")
}

#[post("/test4")]
async fn test_4(
  params: web::Query<IndexQuery>,
  col: web::Data<Collection<User>>,
) -> impl Responder {

  // ruleid: AIK_Rust-nosql-find-injection
  let results = col.find(params.query).await.unwrap();
  do_smth(results);

  HttpResponse::Ok().body("Hello world!")
}

#[post("/ok-test1")]
async fn ok_test_1(
  params: web::Query<IndexQuery>,
  client: web::Data<Client>,
) -> impl Responder {
  let col: Collection<User> = client.database("foo").collection("bar");

  let doc_json = String::from("{{\"$where\": \"this.name == '");
  doc_json.push_str("hello");
  doc_json.push_str("'\"}}");

  let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

  // ok: AIK_Rust-nosql-find-injection
  let results = col.find_one(filter_doc, None).await.unwrap();
  do_smth(results);

  HttpResponse::Ok().body("Hello world!")
}

#[post("/ok-test2")]
async fn test_2(
  req: HttpRequest,
  col: web::Data<Collection<User>>,
) -> Result<String> {
  let params = req.uri().scheme_str().unwrap();
  let doc_json = "{{\"$where\": \"this.name == '" + String::from(params) + "'\"}}";

  let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

  // ruleid: AIK_Rust-nosql-find-injection
  let result = col.find_one(filter_doc, None).await.unwrap();
  do_smth(result);

  Ok("Welcome!")
}

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

async fn api_sqli_check2(col: Collection<User>, req: Request<Incoming>) -> Result<Response<BoxBody>> {
    let whole_body = req.collect().await?.aggregate();
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;

    let doc_json = "{{\"$where\": \"this.name == '" + String::from(data["name"]) + "'\"}}";

    let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

    // ruleid: AIK_Rust-nosql-find-injection
    let result = col.find_one(filter_doc, None).await.unwrap();
    do_smth(result);

    Ok("ok!")
}

async fn api_sqli_check3(col: Collection<User>, req: Request<Incoming>) -> Result<Response<BoxBody>> {
    let whole_body = req.collect().await?.aggregate();
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;

    let query = "SELECT * FROM testing.users WHERE username = '" + String::from(data["name"]) + "';";
    let filter_doc: bson::Document = serde_json::from_str(&query).unwrap();

    // ruleid: AIK_Rust-nosql-find-injection
    let result = col.find_one(filter_doc, None).await.unwrap();
    do_smth(result);

    Ok("ok!")
}

async fn api_sqli_check4(col: Collection<User>, req: Request<Incoming>) -> Result<Response<BoxBody>> {
    let whole_body = req.collect().await?.aggregate();
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;

    let doc_json = String::from("{{\"$where\": \"this.name == '");
    doc_json.push_str(data["name"]);
    doc_json.push_str("'\"}}");

    let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

    // ruleid: AIK_Rust-nosql-find-injection
    let results = col.find(doc_json, None).await.unwrap();
    do_smth(results);

    Ok("ok!")
}

async fn ok_api_sqli_check1(col: Collection<User>, req: Request<Incoming>) -> Result<Response<BoxBody>> {
    let whole_body = req.collect().await?.aggregate();
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;

    let doc_json = String::from("SELECT * FROM testing.users WHERE username = '");
    doc_json.push_str("name");
    doc_json.push_str("';");

    let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

    // ok: AIK_Rust-nosql-find-injection
    let results = col.find(filter_doc, None).await.unwrap();
    do_smth(results);

    Ok("ok!")
}

async fn ok_api_sqli_check2(col: Collection<User>, req: Request<Incoming>) -> Result<Response<BoxBody>> {
    let data = req.uri().scheme_str();

    let doc_json = String::from("{{\"$where\": \"this.name == '");
    doc_json.push_str(data);
    doc_json.push_str("'\"}}");

    let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

    // ok: AIK_Rust-nosql-find-injection
    let results = col.find(filter_doc, None).await.unwrap();
    do_smth(results);

    Ok("ok!")
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
struct Post {
    #[serde(skip_deserializing)]
    id: Option<i64>,
    title: String,
    text: String,
    #[serde(skip_deserializing)]
    published: bool,
}

diesel::table! {
    posts (id) {
        id -> Nullable<BigInt>,
        title -> Text,
        text -> Text,
        published -> Bool,
    }
}

#[post("/test1", data = "<post>")]
async fn test1(client: Client, post: Json<Post>) -> Result<Created<Json<Post>>> {
    let col: Collection<User> = client.database("foo").collection("bar");
    let doc_json = format!(r#"
        {{"$where": "this.name == '{}'"}}
    "#, post.title);

    let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

    // ruleid: AIK_Rust-nosql-find-injection
    let result = col.find(filter_doc, None).await.unwrap();

    let raw_doc = result.current();
    do_smth( raw_doc );

    Ok(Created::new("/").body(post))
}

#[post("/test2")]
async fn test2(user: Collection<User>, post: Form<Post>) -> Option<Json<Post>> {
    let doc_json = "{{\"$where\": \"this.name == '" + String::from(post.title) + "'\"}}";

    let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

    // ruleid: AIK_Rust-nosql-find-injection
    let result = user.find_one(filter_doc, None).await.unwrap();
    do_smth(result);

    Json(Post {})
}

#[get("/test3")]
async fn test3(user: Collection<User>, id: String) -> Option<Json<Post>> {
    let doc_json = format!(r#"
        {{"$where": "this.name == '{}'"}}
    "#, id);

    let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

    // ruleid: AIK_Rust-nosql-find-injection
    let result = user.find(filter_doc, None).await.unwrap();

    let raw_doc = result.current();
    do_smth( raw_doc );

    Json(Post {})
}

#[get("/ok-test1")]
async fn ok_test1(user: Collection<User>, id: String) -> Option<Json<Post>> {
    let doc_json = format!(r#"
        {{"$where": "this.name == '{}'"}}
    "#, "hardcoded-id");

    let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

    // ok: AIK_Rust-nosql-find-injection
    let result = user.find(filter_doc, None).await.unwrap();

    let raw_doc = result.current();
    do_smth( raw_doc );

    Json(Post {})
}

#[get("/ok-test2")]
async fn ok_test2(user: Collection<User>, request: &'r Request<'_>) -> Option<Json<Post>> {
    let value = request.real_ip().unwrap();
    let doc_json = format!(r#"
        {{"$where": "this.name == '{}'"}}
    "#, value);

    let filter_doc: bson::Document = serde_json::from_str(&doc_json).unwrap();

    // ruleid: AIK_Rust-nosql-find-injection
    let result = user.find(filter_doc, None).await.unwrap();

    let raw_doc = result.current();
    do_smth( raw_doc );

    Json(Post {})
}
