use rust_blog_api::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
