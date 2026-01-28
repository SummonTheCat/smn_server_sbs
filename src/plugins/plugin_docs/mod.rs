use std::fs;
use std::path::{Path, PathBuf};

use pulldown_cmark::{Options, Parser, html};

use crate::structures::{PluginBase, SmnRequest, SmnResponse};

pub struct PluginDocs {
    pub root: PathBuf,
    pub template_path: PathBuf,
}

impl PluginDocs {
    pub fn new() -> Self {
        Self {
            root: PathBuf::from("./res/docs"),
            template_path: PathBuf::from("./res/docs_assets/docs_page.html"),
        }
    }

    // ================= Markdown =================

    fn markdown_to_html(markdown: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_FOOTNOTES);

        let parser = Parser::new_ext(markdown, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }

    // ================= Routing =================

    fn is_docs_path(path: &str) -> bool {
        path.starts_with("/docs/")
    }

    fn is_api_docs_path(path: &str) -> bool {
        path.starts_with("/API/Docs")
    }

    fn extract_project(path: &str) -> Option<String> {
        let trimmed = path.trim_start_matches("/docs/");
        trimmed.split('/').next().map(|s| s.to_string())
    }

    fn resolve_md_path(&self, path: &str) -> Option<PathBuf> {
        let trimmed = path.trim_start_matches("/docs/");
        let mut parts = trimmed.split('/');

        let project = parts.next()?;
        let mut file_path = self.root.join(project);

        for part in parts {
            file_path.push(part);
        }

        match file_path.extension().and_then(|e| e.to_str()) {
            None => {
                file_path.set_extension("md");
            }
            Some("md") => {}
            Some(_) => {
                file_path.set_extension("md");
            }
        }

        Some(file_path)
    }

    // ================= Template =================

    fn load_template(&self) -> Result<String, ()> {
        fs::read_to_string(&self.template_path).map_err(|_| ())
    }

    fn inject(template: String, sidebar: &str, content: &str) -> String {
        template
            .replace("</ElemSidebar>", sidebar)
            .replace("</ElemMDContent>", content)
    }

    // ================= Sidebar =================

    // ================= Sidebar =================

    fn build_sidebar(project: &str, project_root: &Path, current_route: &str) -> String {
        let mut out = String::new();

        out.push_str(&format!("<div class=\"docs-project\">{}</div>\n", project));

        Self::walk_dir(project, project_root, project_root, current_route, &mut out);

        out
    }

    fn walk_dir(project: &str, base: &Path, current: &Path, current_route: &str, out: &mut String) {
        let entries = match fs::read_dir(current) {
            Ok(e) => e,
            Err(_) => return,
        };

        let mut files = Vec::<PathBuf>::new();
        let mut dirs = Vec::<PathBuf>::new();

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                files.push(path);
            }
        }

        // ---------- Sort deterministically ----------

        files.sort_by(|a, b| {
            let a_name = a.file_stem().unwrap().to_string_lossy().to_lowercase();
            let b_name = b.file_stem().unwrap().to_string_lossy().to_lowercase();

            match (a_name.as_str(), b_name.as_str()) {
                ("readme", "readme") => std::cmp::Ordering::Equal,
                ("readme", _) => std::cmp::Ordering::Less,
                (_, "readme") => std::cmp::Ordering::Greater,
                _ => a_name.cmp(&b_name),
            }
        });

        dirs.sort_by(|a, b| {
            a.file_name()
                .unwrap()
                .to_string_lossy()
                .to_lowercase()
                .cmp(&b.file_name().unwrap().to_string_lossy().to_lowercase())
        });

        // Build stem → file map
        let mut file_map = std::collections::HashMap::new();
        for file in files {
            if let Some(stem) = file.file_stem().and_then(|s| s.to_str()) {
                file_map.insert(stem.to_string(), file);
            }
        }

        out.push_str("<ul class=\"docs-tree\">\n");

        // ---------- Render standalone files first ----------
        let mut standalone = Vec::new();
        for (stem, path) in &file_map {
            if !dirs.iter().any(|d| d.file_name().unwrap() == stem.as_str()) {
                standalone.push(path.clone());
            }
        }

        // Preserve README-first + alpha ordering
        standalone.sort_by(|a, b| {
            let a_name = a.file_stem().unwrap().to_string_lossy().to_lowercase();
            let b_name = b.file_stem().unwrap().to_string_lossy().to_lowercase();

            match (a_name.as_str(), b_name.as_str()) {
                ("readme", "readme") => std::cmp::Ordering::Equal,
                ("readme", _) => std::cmp::Ordering::Less,
                (_, "readme") => std::cmp::Ordering::Greater,
                _ => a_name.cmp(&b_name),
            }
        });

        for file in standalone {
            out.push_str("<li class=\"docs-file\">\n");
            Self::render_file_link(project, base, &file, current_route, out, "file");
            out.push_str("</li>\n");

            if let Some(stem) = file.file_stem().and_then(|s| s.to_str()) {
                file_map.remove(stem);
            }
        }

        // ---------- Render directories ----------
        for dir in dirs {
            let name = dir.file_name().unwrap().to_string_lossy().to_string();
            let header_file = file_map.remove(&name);

            out.push_str("<li class=\"docs-dir\">\n");

            if let Some(md) = header_file {
                Self::render_file_link(project, base, &md, current_route, out, "dir-link");
            } else {
                out.push_str(&format!("<div class=\"dir-label\">{}</div>\n", name));
            }

            out.push_str("<ul class=\"docs-subtree\">\n");
            Self::walk_dir(project, base, &dir, current_route, out);
            out.push_str("</ul>\n</li>\n");
        }

        out.push_str("</ul>\n");
    }

    fn render_file_link(
        project: &str,
        base: &Path,
        path: &Path,
        current_route: &str,
        out: &mut String,
        class: &str,
    ) {
        let name = path.file_stem().unwrap().to_string_lossy();

        let rel = path.strip_prefix(base).unwrap();
        let mut link = rel.to_string_lossy().replace('\\', "/");
        link = link.trim_end_matches(".md").to_string();

        let full_route = format!("/docs/{}/{}", project, link);
        let active = full_route == current_route;

        let final_class = if active {
            format!("{} active", class)
        } else {
            class.to_string()
        };

        out.push_str(&format!(
            "<a class=\"{}\" href=\"{}\">{}</a>\n",
            final_class, full_route, name
        ));
    }
}

impl PluginBase for PluginDocs {
    fn name(&self) -> &str {
        "docs"
    }

    fn init(&mut self) {
        println!(
            "Docs plugin initialized\n- docs root: {:?}\n- template: {:?}",
            self.root, self.template_path
        );
    }

    fn can_serve(&self, request: &SmnRequest) -> bool {
        request.method == "GET"
            && (Self::is_docs_path(&request.path) || Self::is_api_docs_path(&request.path))
    }

    fn serve(&self, request: &SmnRequest) -> SmnResponse {
        if Self::is_api_docs_path(&request.path) {
            return SmnResponse::new(200, "OK", b"API COMING SOON".to_vec())
                .with_header("Content-Type", "text/plain; charset=utf-8");
        }

        let project = match Self::extract_project(&request.path) {
            Some(p) => p,
            None => {
                return SmnResponse::new(400, "Bad Request", b"Invalid docs path".to_vec())
                    .with_header("Content-Type", "text/plain");
            }
        };

        let md_path = match self.resolve_md_path(&request.path) {
            Some(p) => p,
            None => {
                return SmnResponse::new(404, "Not Found", b"Documentation not found".to_vec())
                    .with_header("Content-Type", "text/plain");
            }
        };

        let markdown = match fs::read_to_string(&md_path) {
            Ok(c) => c,
            Err(_) => {
                return SmnResponse::new(404, "Not Found", b"Documentation not found".to_vec())
                    .with_header("Content-Type", "text/plain");
            }
        };

        let md_html = Self::markdown_to_html(&markdown);

        let project_root = self.root.join(&project);
        let current_route = request.path.trim_end_matches(".md");
        let sidebar_html = Self::build_sidebar(&project, &project_root, current_route);

        let template = match self.load_template() {
            Ok(t) => t,
            Err(_) => {
                return SmnResponse::new(
                    500,
                    "Internal Server Error",
                    b"Docs template missing".to_vec(),
                )
                .with_header("Content-Type", "text/plain");
            }
        };

        let final_html = Self::inject(template, &sidebar_html, &md_html);

        SmnResponse::new(200, "OK", final_html.into_bytes())
            .with_header("Content-Type", "text/html; charset=utf-8")
    }
}
