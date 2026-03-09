use axum::{
    routing::get,
    response::{Html, IntoResponse},
    http::header,
    Router,
};
use clap::Parser;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use tower::ServiceBuilder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to serve assets from
    #[arg(default_value = ".")]
    directory: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = 8000)]
    port: u16,
}

async fn serve_html() -> Html<&'static str> {
    Html(include_str!("glb-viewer.html"))
}

async fn serve_css() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css")], include_str!("glb-viewer.css"))
}

async fn serve_js() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/javascript")], include_str!("glb-viewer.js"))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    
    // Handle Bazel internal execution environment
    if let Ok(workspace_dir) = std::env::var("BUILD_WORKSPACE_DIRECTORY") {
        std::env::set_current_dir(&workspace_dir).unwrap();
    }

    let data_dir = Path::new(&args.directory).canonicalize().unwrap_or_else(|_| PathBuf::from(&args.directory));

    println!("Data directory:   {}", data_dir.display());
    println!("Serving at http://localhost:{}", args.port);
    println!("To view a model: http://localhost:{}/glb-viewer.html?model=path/to/model.glb", args.port);

    let data_service = ServeDir::new(&data_dir);
    
    // Serve embedded static files, plus any assets from data_dir
    let app = Router::new()
        .route("/glb-viewer.html", get(serve_html))
        .route("/glb-viewer.css", get(serve_css))
        .route("/glb-viewer.js", get(serve_js))
        .route("/", get(serve_html))
        .fallback_service(data_service)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    println!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
