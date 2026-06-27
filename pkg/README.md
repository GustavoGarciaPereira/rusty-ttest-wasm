# 🦀 Rusty TTest Wasm

Suite de ferramentas científicas de alta performance rodando no navegador via **Rust** + **WebAssembly**.

![Rust](https://img.shields.io/badge/Core-Rust_2021-orange)
![WebAssembly](https://img.shields.io/badge/Target-Wasm32-yellow)
![UI](https://img.shields.io/badge/UI-Pico.css-blue)
![Deploy](https://img.shields.io/badge/Deploy-GitHub_Pages-green)
![Tests](https://img.shields.io/badge/Tests-22%2F22-brightgreen)

🌐 **[Acessar aplicação](https://gustavogarciapereira.github.io/rusty-ttest-wasm/)**

---

## 🧭 Ferramentas

| # | Página | Descrição |
|---|---|---|
| 🏠 | `index.html` | Menu de navegação com 8 cards |
| 📊 | [`ttest.html`](ttest.html) | Testes-t estatísticos (One-sample, Welch, Pareado) |
| 🧲 | [`campo-eletrico.html`](campo-eletrico.html) | Simulação 2D de campo elétrico com Canvas |
| 🌊 | [`poiseuille.html`](poiseuille.html) | Poiseuille — perfil parabólico (analítico vs numérico) |
| 🎨 | [`poiseuille-visual.html`](poiseuille-visual.html) | Poiseuille — mapa de cores + partículas animadas |
| 📐 | [`couette.html`](couette.html) | Couette — perfil linear (analítico vs numérico) |
| 🎬 | [`couette-visual.html`](couette-visual.html) | Couette — mapa de cores + partículas animadas |
| 📈 | [`backward-step.html`](backward-step.html) | Degrau — perfis u(y) e p(x) para validação |
| 🔬 | [`backward-step-visual.html`](backward-step-visual.html) | Degrau — solver SIMPLE 2D + 200 partículas traçadoras |

---

## ✨ Funcionalidades

### 📊 Testes-t Estatísticos
- 3 tipos: One-sample, Independentes (Welch), Pareado
- P-value bicaudal com `statrs` e marcadores de significância (`*`, `**`, `***`)
- Upload CSV via drag & drop, detecção de separador, mapeamento dinâmico de colunas
- 100% client-side — zero servidor

### 🧲 Campo Elétrico 2D
- Motor físico em Rust: lei de Coulomb com softening, soma vetorial
- Saída `Uint8ClampedArray` nativa (zero-cópia para o Canvas)
- Coloração HSV: matiz = direção, brilho = √intensidade
- Cargas aleatórias, toggle claro/escuro 🌙☀️

### 🌊📐🎨🎬 Escoamentos Laminares (Poiseuille & Couette)
- Solução analítica exata + solver numérico via **diferenças finitas + TDMA**
- 4 páginas: gráfico 2D e visualização interativa (mapa de cores + partículas) para cada
- Animação em tempo real: partículas usam `velocity_analytical` via WASM por frame
- Sliders com auto-update, controle de velocidade, pause/retomar

### 🔬📈 Backward-Facing Step (Degrau Retangular)
- **Solver SIMPLE 2D** completo: malha deslocada, upwind, Gauss-Seidel
- 200 partículas traçadoras com interpolação bilinear em JS
- Página de perfis: u(y) em 4 estações + p(x) na linha central
- Proteção de malha (máx 120×60) para evitar estouro de memória WASM

---

## 🏗️ Arquitetura

```
src/
├── lib.rs              → Testes-t + declaração dos 4 módulos
├── simulation.rs       → Campo elétrico (Charge + solver HSV + bridge WASM)
├── poiseuille.rs       → Poiseuille (analítico + TDMA + bridge WASM)
├── couette.rs          → Couette (analítico + TDMA + bridge WASM)
└── backward_step.rs    → SIMPLE 2D CFD solver (386 linhas)

frontend/
├── index.html                  → Menu com 8 cards
├── ttest.html                  → Testes-t
├── campo-eletrico.html         → Campo elétrico
├── poiseuille.html / poiseuille-visual.html
├── couette.html / couette-visual.html
└── backward-step.html / backward-step-visual.html

pkg/                    → Artefatos WASM commitados
```

### Stack

| Camada | Tecnologia |
|---|---|
| Motor matemático | Rust 2021 (`statrs`, `serde`, `serde_json`, `wasm-bindgen`, `js-sys`) |
| CFD solver | Diferenças finitas + TDMA + SIMPLE com malha deslocada |
| Compilação | `wasm-pack build --target web` |
| Bridge JS↔WASM | ES Modules + `Uint8ClampedArray` (zero-cópia) |
| Frontend | Vanilla JS + Canvas 2D + Pico.css |
| Deploy | GitHub Actions → GitHub Pages |

---

## 💻 Desenvolvimento local

```bash
git clone https://github.com/GustavoGarciaPereira/rusty-ttest-wasm.git
cd rusty-ttest-wasm
wasm-pack build --target web
python -m http.server 8000
# Acesse http://localhost:8000
```

### Testes

```bash
cargo test   # 22 testes: t-test (13) + poiseuille (4) + couette (3) + backward_step (2)
```

O core matemático é isolado do `#[wasm_bindgen]`, permitindo validação nativa em x86.

---

## 🚀 Deploy

Push na `main` dispara GitHub Actions: `wasm-pack build` → upload da raiz → GitHub Pages.  
Os artefatos `pkg/` são commitados para evitar problemas de cache/`.gitignore` na action de upload.
