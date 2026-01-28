use eframe::egui;
use tree_sitter::{Parser, Language, Node};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("SPFx Recon: One-Stop Shop", options, Box::new(|_cc| Box::new(ReconApp::default())))
}

struct ReconApp {
    results: Vec<AnalysisReport>,
}

struct AnalysisReport {
    filename: String,
    risk_color: egui::Color32,
    summary: String,
    ast_preview: Vec<String>,
}

impl Default for ReconApp {
    fn default() -> Self {
        Self { results: vec![] }
    }
}

impl eframe::App for ReconApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🚀 SPFx Semantic Synthesis Tool");
            ui.label("Drop any file to determine the 'Shape' of the code.");
            ui.separator();

            // 1. Handle Drop
            if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
                let dropped = ctx.input(|i| i.raw.dropped_files.clone());
                for file in dropped {
                    if let Some(path) = file.path { self.analyze_file(path); }
                }
            }

            // 2. Display Result Cards
            egui::ScrollArea::vertical().show(ui, |ui| {
                for report in &self.results {
                    ui.group(|ui| {
                        ui.colored_label(report.risk_color, format!("FILE: {}", report.filename));
                        ui.label(egui::RichText::new(&report.summary).strong());
                        
                        ui.collapsing("View Abstract Syntax Tree (AST) Shape", |ui| {
                            for line in &report.ast_preview {
                                ui.monospace(line);
                            }
                        });
                    });
                    ui.add_space(10.0);
                }
            });
        });
    }
}

impl ReconApp {
    fn analyze_file(&mut self, path: PathBuf) {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_typescript::language_typescript()).unwrap();

        let code = fs::read_to_string(&path).unwrap_or_default();
        let tree = parser.parse(&code, None).unwrap();
        let root = tree.root_node();

        // --- Semantic Analysis Logic ---
        let mut funcs = 0; let mut ifs = 0;
        let mut has_api = false; let mut has_dom = false;
        let mut ast_preview = Vec::new();

        self.traverse_and_review(root, &code, &mut funcs, &mut ifs, &mut has_api, &mut has_dom, &mut ast_preview, 0);

        // --- Determine Shape & Risk ---
        let mut risk_color = egui::Color32::GREEN;
        let mut tags = vec![];

        if has_api { risk_color = egui::Color32::from_rgb(255, 165, 0); tags.push("📡 API-ACTIVE"); } // Orange
        if ifs > 5 { risk_color = egui::Color32::RED; tags.push("🧠 COMPLEX-LOGIC"); }
        if has_dom { risk_color = egui::Color32::RED; tags.push("⚠️ DOM-MANIPULATION"); }

        let summary = format!("Tags: {:?} | Methods: {} | Decision Points: {}", tags, funcs, ifs);

        self.results.insert(0, AnalysisReport {
            filename: path.file_name().unwrap().to_string_lossy().to_string(),
            risk_color,
            summary,
            ast_preview,
        });
    }

    fn traverse_and_review(&self, node: Node, code: &str, funcs: &mut i32, ifs: &mut i32, has_api: &mut bool, has_dom: &mut bool, ast: &mut Vec<String>, depth: usize) {
        let kind = node.kind();
        
        // Build AST visualization
        if depth < 5 { // Limit depth for UI readability
            ast.push(format!("{}{}", "  ".repeat(depth), kind));
        }

        // Semantic Rules
        if kind == "method_definition" || kind == "function_declaration" { *funcs += 1; }
        if kind == "if_statement" { *ifs += 1; }
        
        if let Ok(text) = node.utf8_text(code.as_bytes()) {
            if text.contains("spHttpClient") || text.contains("graphClient") { *has_api = true; }
            if text.contains("document.") || text.contains(".innerHTML") { *has_dom = true; }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.traverse_and_review(child, code, funcs, ifs, has_api, has_dom, ast, depth + 1);
        }
    }
}