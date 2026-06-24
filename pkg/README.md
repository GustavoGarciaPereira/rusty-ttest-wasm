# 🦀 Rusty TTest Wasm

Calculadora estatística de alta performance + simulador de campo elétrico 2D — tudo rodando no navegador via **Rust** + **WebAssembly**.

![Rust](https://img.shields.io/badge/Core-Rust_2021-orange)
![WebAssembly](https://img.shields.io/badge/Target-Wasm32-yellow)
![UI](https://img.shields.io/badge/UI-Pico.css-blue)
![Deploy](https://img.shields.io/badge/Deploy-GitHub_Pages-green)

🌐 **[Acessar aplicação](https://gustavogarciapereira.github.io/rusty-ttest-wasm/)**

---

## 🧭 Ferramentas

| Página | Descrição |
|---|---|
| `index.html` | Menu de navegação |
| [`ttest.html`](https://gustavogarciapereira.github.io/rusty-ttest-wasm/ttest.html) | 📊 Testes-t estatísticos (One-sample, Welch, Pareado) |
| [`campo-eletrico.html`](https://gustavogarciapereira.github.io/rusty-ttest-wasm/campo-eletrico.html) | 🧲 Simulação 2D de campo elétrico com Canvas |

---

## ✨ Funcionalidades

### 📊 Testes-t Estatísticos

- **3 tipos de teste:** One-sample, Duas amostras independentes (Welch) e Pareado
- **P-value bicaudal** com distribuição t de Student (crate `statrs`) e marcadores de significância (`*`, `**`, `***`)
- **Upload de CSV** via drag & drop com detecção automática de separador (`,` ou `;`)
- **Mapeamento dinâmico de colunas** — relacione colunas do CSV com as variáveis do teste
- **100% client-side** — sem servidor, sem telemetria

### 🧲 Simulação de Campo Elétrico

- **Motor físico em Rust** — lei de Coulomb com softening para evitar singularidades
- **Renderização direta no Canvas** — saída `Uint8ClampedArray` nativa, sem serialização
- **Coloração HSV** — matiz = direção do campo, brilho = intensidade (curva √ para realçar regiões fracas)
- **Cargas aleatórias** — botão para adicionar cargas com posição e magnitude randômicas
- **Toggle claro/escuro** 🌙☀️ com persistência em `localStorage`

---

## 🏗️ Arquitetura

```
src/
├── lib.rs              → Módulo de testes-t (One-sample, Welch, Pareado) + exports WASM
└── simulation.rs       → Struct Charge + solver de campo elétrico + bridge WASM

frontend/
├── index.html          → Menu com cards para as ferramentas
├── ttest.html          → UI completa dos testes-t (Pico.css + Vanilla JS)
└── campo-eletrico.html → UI do simulador de campo elétrico (Pico.css + Canvas)

pkg/                    → Artefatos WASM commitados (deploy direto no GitHub Pages)
```

### Stack

| Camada | Tecnologia |
|---|---|
| Motor matemático | Rust 2021 (`statrs`, `serde`, `serde_json`) |
| Compilação WASM | `wasm-bindgen` + `wasm-pack` |
| Bridge JS↔WASM | `js-sys::Uint8ClampedArray` (zero-cópia) |
| Frontend | Vanilla JS + ES Modules + Pico.css |
| Deploy | GitHub Actions → GitHub Pages |

---

## 💻 Desenvolvimento local

```bash
# 1. Clone
git clone https://github.com/GustavoGarciaPereira/rusty-ttest-wasm.git
cd rusty-ttest-wasm

# 2. Compile Rust → WASM
wasm-pack build --target web

# 3. Sirva localmente (necessário para módulos ES)
python -m http.server 8000

# 4. Acesse http://localhost:8000
```

### Testes

```bash
cargo test
```

O core matemático é isolado do `#[wasm_bindgen]`, permitindo rodar a suíte de validação nativamente em x86.

---

## 🚀 Deploy (GitHub Pages)

O deploy é automático via GitHub Actions — push na branch `main` dispara:

1. Checkout do código + instalação do Rust e `wasm-pack`
2. `wasm-pack build --target web`
3. Upload da raiz (`index.html`, `ttest.html`, `campo-eletrico.html`, `pkg/`) para o GitHub Pages

Os artefatos `pkg/` também são commitados para evitar problemas de cache/respeito a `.gitignore` pela action de upload.
