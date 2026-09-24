#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = api::build_app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("API listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
