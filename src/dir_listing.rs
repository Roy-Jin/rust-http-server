use std::path::Path;

use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use tokio::fs;

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;
    
    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub async fn serve_dir_listing(_root: &Path, dir_path: &Path, request_path: &str) -> Response {
    let mut entries = match fs::read_dir(dir_path).await {
        Ok(e) => e,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
    };

    let mut items = Vec::new();

    while let Some(entry) = entries.next_entry().await.unwrap() {
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if file_name_str.starts_with('.') {
            continue;
        }

        let is_dir = entry.file_type().await.unwrap().is_dir();
        let metadata = entry.metadata().await.unwrap();
        let size = metadata.len();
        
        items.push((file_name_str.to_string(), is_dir, size));
    }

    let mut html = String::new();
    html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Directory: "#);
    html.push_str(&escape_html(request_path));
    html.push_str(r#"</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        html, body {
            height: 100%;
            width: 100%;
            overflow: hidden;
        }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background: linear-gradient(135deg, #0f0f0f 0%, #1a1a2e 50%, #16213e 100%);
            color: #e8e8e8;
            display: flex;
            flex-direction: column;
        }
        .container {
            flex: 1;
            display: flex;
            flex-direction: column;
            height: 100%;
            overflow: hidden;
        }
        .header {
            background: rgba(20, 20, 30, 0.95);
            backdrop-filter: blur(20px);
            border-bottom: 1px solid rgba(255, 255, 255, 0.08);
            padding: 24px 40px;
            flex-shrink: 0;
        }
        h1 {
            font-size: 28px;
            font-weight: 700;
            color: #ffffff;
            margin-bottom: 8px;
            letter-spacing: -0.5px;
            display: flex;
            align-items: center;
            gap: 12px;
        }
        h1::before {
            content: '';
            display: inline-block;
            width: 4px;
            height: 28px;
            background: linear-gradient(180deg, #667eea 0%, #764ba2 100%);
            border-radius: 2px;
        }
        .path {
            color: #888;
            font-size: 14px;
            font-family: 'SF Mono', 'Fira Code', 'Consolas', monospace;
            padding-left: 16px;
            word-break: break-all;
        }
        .content-wrapper {
            flex: 1;
            overflow-y: auto;
            overflow-x: hidden;
            padding: 0;
        }
        .directory-table {
            padding: 16px 24px;
        }
        .file-item {
            display: flex;
            align-items: center;
            padding: 14px 20px;
            margin-bottom: 4px;
            background: rgba(30, 30, 40, 0.6);
            border-radius: 12px;
            cursor: pointer;
            transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
            border: 1px solid transparent;
        }
        .file-item:hover {
            background: rgba(50, 50, 70, 0.8);
            border-color: rgba(102, 126, 234, 0.3);
            transform: translateX(4px);
        }
        .file-item:active {
            transform: translateX(2px);
        }
        .file-item .name-cell {
            flex: 1;
            display: flex;
            align-items: center;
            gap: 14px;
            min-width: 0;
        }
        .file-item .icon {
            font-size: 20px;
            width: 28px;
            text-align: center;
            flex-shrink: 0;
        }
        .file-item .file-link,
        .file-item .dir-link,
        .file-item .parent-link {
            color: #e8e8e8;
            text-decoration: none;
            font-weight: 500;
            font-size: 15px;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .file-item:hover .file-link,
        .file-item:hover .dir-link,
        .file-item:hover .parent-link {
            color: #667eea;
        }
        .file-item .size {
            color: #666;
            font-family: 'SF Mono', 'Fira Code', 'Consolas', monospace;
            font-size: 13px;
            padding-left: 20px;
            flex-shrink: 0;
            min-width: 80px;
            text-align: right;
        }
        .file-item .type {
            color: #667eea;
            font-size: 12px;
            font-weight: 600;
            padding-left: 20px;
            flex-shrink: 0;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        .folder-icon { color: #667eea; }
        .file-icon { color: #48bb78; }
        .parent-icon { color: #ed8936; }
        .footer {
            background: rgba(20, 20, 30, 0.95);
            backdrop-filter: blur(20px);
            border-top: 1px solid rgba(255, 255, 255, 0.08);
            padding: 16px 40px;
            text-align: center;
            flex-shrink: 0;
        }
        .author {
            color: #666;
            font-size: 13px;
        }
        .author a {
            color: #667eea;
            text-decoration: none;
            font-weight: 500;
            transition: color 0.2s;
        }
        .author a:hover {
            color: #764ba2;
        }
        ::-webkit-scrollbar {
            width: 8px;
        }
        ::-webkit-scrollbar-track {
            background: rgba(0, 0, 0, 0.2);
        }
        ::-webkit-scrollbar-thumb {
            background: rgba(102, 126, 234, 0.3);
            border-radius: 4px;
        }
        ::-webkit-scrollbar-thumb:hover {
            background: rgba(102, 126, 234, 0.5);
        }
        @media (max-width: 768px) {
            .header { padding: 20px 24px; }
            .directory-table { padding: 12px 16px; }
            .file-item { padding: 12px 16px; }
            h1 { font-size: 22px; }
            .file-item .file-link,
            .file-item .dir-link,
            .file-item .parent-link { font-size: 14px; }
        }
        @media (max-width: 480px) {
            .header { padding: 16px 20px; }
            .directory-table { padding: 8px 12px; }
            .file-item { padding: 10px 14px; margin-bottom: 2px; }
            h1 { font-size: 18px; }
            .path { font-size: 12px; }
            .file-item .icon { font-size: 18px; width: 24px; }
            .file-item .file-link,
            .file-item .dir-link,
            .file-item .parent-link { font-size: 13px; }
            .file-item .size { font-size: 11px; min-width: 60px; }
            .file-item .type { font-size: 10px; }
            .footer { padding: 12px 20px; }
            .author { font-size: 11px; }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>HTTP SERVER</h1>
            <div class="path">"#);
    html.push_str(&escape_html(request_path));
    html.push_str(r#"</div>
        </div>
        <div class="content-wrapper">
            <div class="directory-table">"#);

    if request_path != "/" {
        let parent = Path::new(request_path).parent().unwrap_or(Path::new("/"));
        let parent_str = parent.to_string_lossy().to_string();
        html.push_str(&format!(r#"
            <div class="file-item">
                <div class="name-cell">
                    <span class="icon parent-icon">⬆</span>
                    <span class="parent-link" data-path="{}">..</span>
                </div>
                <span class="type">DIR</span>
            </div>"#,
            escape_html(&parent_str)
        ));
    }

    for (name, is_dir, size) in &items {
        let href = if *is_dir {
            format!("{}/{}", request_path.trim_end_matches('/'), escape_html(name))
        } else {
            format!("{}/{}", request_path.trim_end_matches('/'), escape_html(name))
        };

        if *is_dir {
            html.push_str(&format!(r#"
            <div class="file-item">
                <div class="name-cell">
                    <span class="icon folder-icon">📁</span>
                    <span class="dir-link" data-path="{}">{}</span>
                </div>
                <span class="type">DIR</span>
            </div>"#,
                escape_html(&href),
                escape_html(name)
            ));
        } else {
            html.push_str(&format!(r#"
            <div class="file-item">
                <div class="name-cell">
                    <span class="icon file-icon">📄</span>
                    <span class="file-link" data-path="{}">{}</span>
                </div>
                <span class="size">{}</span>
            </div>"#,
                escape_html(&href),
                escape_html(name),
                format_size(*size)
            ));
        }
    }

    html.push_str(r#"
            </div>
        </div>
        <div class="footer">
            <div class="author">Created by <a href="https://github.com/Roy-Jin" target="_blank">GitHub/Roy-Jin</a></div>
        </div>
    </div>
    <script>
        (function() {
            var wrapper = document.querySelector('.directory-table');
            if (!wrapper) return;
            wrapper.addEventListener('click', function(e) {
                var item = e.target.closest('.file-item');
                if (!item) return;
                var linkElem = item.querySelector('[data-path]');
                if (!linkElem) return;
                var path = linkElem.getAttribute('data-path');
                if (path) {
                    window.location.href = path;
                }
            });
        })();
    </script>
</body>
</html>"#);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    (StatusCode::OK, headers, html).into_response()
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#x27;")
}
