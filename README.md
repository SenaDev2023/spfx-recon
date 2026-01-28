# SPFx Recon: Semantic Synthesis Engine

**SPFx Recon** is a systems-level static analysis tool for deep semantic review of SharePoint Framework (SPFx) components. It uses high-fidelity parsing to detect architectural patterns and security risks that regex-based scanners miss.

---

## Core Capabilities

- **AST-Based Traversal**  
  Walks the Abstract Syntax Tree to analyze real code structure, not string matches.

- **Risk Scoring**  
  Flags unsafe DOM manipulation (`innerHTML`, `document.*`) and unauthorized API access patterns.

- **Complexity Mapping**  
  Measures cognitive load via decision-point density (conditionals) and method counts.

- **Modern UI**  
  Real-time, hardware-accelerated analysis dashboard built with `egui`.

---

## Technology Stack

- **Engine:** Rust (memory-safe, high-concurrency)
- **Parser:** `tree-sitter-typescript` (grammar-based incremental parsing)
- **GUI:** `eframe` / `egui` (immediate-mode rendering)

---

## Usage

### Build
```bash
cargo build --release


## 🔍 Interface & AST Analysis

### Application Dashboard
![Main UI](assets/ui-main.png)

### Semantic Deep Dive
This view shows the `tree-sitter` parser identifying specific TypeScript nodes like `import_statement` and `namespace_import`.
![AST Detail](assets/ast-detail.png)



