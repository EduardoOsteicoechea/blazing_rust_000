// main.rs
use axum::{
    routing::get,
    response::Html,
    Router,
};
use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {    
    let app = Router::new().route("/", get(root_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 7000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Blazing fast API listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> Html<&'static str> {
    Html("
        <!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"UTF-8\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            <title>Blazing Fast Rust API</title>
            <style>
                body {
                    font-family: 'Inter', sans-serif;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    min-height: 100vh;
                    margin: 0;
                    background-color: #1a202c; /* Dark background */
                    color: #e2e8f0; /* Light text */
                    text-align: center;
                }
                .container {
                    padding: 2rem;
                    background-color: #2d3748; /* Slightly lighter dark background */
                    border-radius: 0.75rem; /* Rounded corners */
                    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                }
                h1 {
                    color: #63b3ed; /* Blue for emphasis */
                }
                p {
                    margin-top: 1rem;
                }
            </style>
            <link href=\"https://fonts.googleapis.com/css2?family=Inter:wght@400;700&display=swap\" rel=\"stylesheet\">
            <link rel=\"stylesheet\" href=\"global.css\">
            <script type=\"module\" src=\"global.js\"></script>
        </head>
        <body>
            <div class=\"container\">
                <h1>✨ Hello from Blazing Fast Rust! ✨</h1>
                <p>This HTML was served directly from an Axum API.</p>
                <p>It's optimized for speed, even with multiple hops.</p>
            </div>
        </body>
        </html>
    ")
}