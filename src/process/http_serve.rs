use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{info, warn};
// Path vs PathBuf => str vs String
// state 承载 path
#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}
// 1.
pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let dir_service = ServeDir::new(path.clone())
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();
    info!("Serving {:?} on http://{}", path, addr);
    let state = HttpServeState { path: path.clone() };

    let app = Router::new()
        .nest_service("/tower", dir_service)
        .route("/*path", get(file_handle))
        .with_state(Arc::new(state));

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn file_handle(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        return (StatusCode::NOT_FOUND, format!("File  {:?} not found", p));
    } else {
        // TODO: test p is a directory
        // if it is a directory, list all files/subdirectories
        // as <li><a href="/path/to/file">file</a></li>
        // <html><body><ul>...</ul></body></html>

        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                // 二进制文件读取不到，因为尝试转换成string
                warn!("Error reading file: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error reading file".to_string(),
                )
            }
        }
    }

    // todo!()
}
// path match//

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handle() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handle(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
//TODO: directory index support