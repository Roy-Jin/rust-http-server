use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use mime_guess::{from_path, mime};
use tokio::fs;

use crate::state::AppState;

pub async fn not_found_response(state: &AppState) -> (StatusCode, Response) {
    let custom_404_path = state.root.join(&state.not_found_page);
    if custom_404_path.exists() {
        if let Ok(content) = fs::read(&custom_404_path).await {
            let mime = from_path(&custom_404_path).first_or_octet_stream();
            let mut headers = HeaderMap::new();
            let content_type = if mime.type_() == mime::TEXT {
                format!("{}; charset=utf-8", mime)
            } else {
                mime.to_string()
            };
            headers.insert("Content-Type", content_type.parse().unwrap());
            return (StatusCode::NOT_FOUND, (StatusCode::NOT_FOUND, headers, content).into_response());
        }
    }

    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 Not Found</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        html, body {
            height: 100%;
            width: 100%;
        }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background: linear-gradient(135deg, #0f0f0f 0%, #1a1a2e 50%, #16213e 100%);
            color: #e8e8e8;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
        }
        .container {
            text-align: center;
            padding: 3rem 2rem;
            background: rgba(30, 30, 40, 0.8);
            backdrop-filter: blur(20px);
            border: 1px solid rgba(255, 255, 255, 0.08);
            border-radius: 16px;
            max-width: 500px;
            margin: 1rem;
        }
        .error-code {
            font-size: 8rem;
            font-weight: 900;
            background: linear-gradient(180deg, #667eea 0%, #764ba2 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            margin-bottom: 1rem;
            letter-spacing: -0.05em;
        }
        h1 {
            font-size: 1.75rem;
            font-weight: 700;
            color: #ffffff;
            margin-bottom: 1rem;
        }
        .description {
            color: #888;
            font-size: 1rem;
            margin-bottom: 2rem;
            line-height: 1.6;
        }
        .go-back-btn {
            display: inline-block;
            padding: 0.875rem 2rem;
            background: linear-gradient(180deg, #667eea 0%, #764ba2 100%);
            color: #ffffff;
            text-decoration: none;
            font-weight: 600;
            font-size: 0.95rem;
            border-radius: 8px;
            transition: all 0.2s;
            cursor: pointer;
            border: none;
        }
        .go-back-btn:hover {
            transform: translateY(-2px);
            box-shadow: 0 10px 20px -10px rgba(102, 126, 234, 0.5);
        }
        .go-back-btn:active {
            transform: translateY(0);
        }
        @media (max-width: 768px) {
            .error-code {
                font-size: 5rem;
            }
            h1 {
                font-size: 1.5rem;
            }
            .container {
                padding: 2rem 1.5rem;
            }
        }
        @media (max-width: 480px) {
            .error-code {
                font-size: 4rem;
            }
            h1 {
                font-size: 1.25rem;
            }
            .description {
                font-size: 0.9rem;
            }
            .go-back-btn {
                padding: 0.75rem 1.5rem;
                font-size: 0.9rem;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="error-code">404</div>
        <h1>Page Not Found</h1>
        <p class="description">The requested resource was not found on this server.</p>
        <button class="go-back-btn" onclick="window.location.href='/'">
            Go Back Home
        </button>
    </div>
</body>
</html>"#;
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    (StatusCode::NOT_FOUND, (StatusCode::NOT_FOUND, headers, html).into_response())
}
