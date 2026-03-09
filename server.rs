use axum::{
    routing::get_service,
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    
    // Handle Bazel internal execution environment
    if let Ok(workspace_dir) = std::env::var("BUILD_WORKSPACE_DIRECTORY") {
        std::env::set_current_dir(&workspace_dir).unwrap();
    }

    let viewer_dir = std::env::current_dir().unwrap();
    let data_dir = Path::new(&args.directory).canonicalize().unwrap_or_else(|_| PathBuf::from(&args.directory));

    println!("Viewer directory: {}", viewer_dir.display());
    println!("Data directory:   {}", data_dir.display());
    println!("Serving at http://localhost:{}", args.port);
    println!("To view a model: http://localhost:{}/glb-viewer.html?model=path/to/model.glb", args.port);

    // Chained service: try viewer_dir first, then data_dir
    let viewer_service = ServeDir::new(&viewer_dir);
    let data_service = ServeDir::new(&data_dir);
    
    // Axum fallback: if not in viewer_dir, try data_dir
    let app = Router::new()
        .fallback_service(
            viewer_service.clone().fallback(data_service)
        )
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    println!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
