# 🦀 Projeto: rusty-ttest-wasm
Calculadora estatística de amostragem de alta performance focada nos 3 principais Testes-t.
Stack: Rust, WebAssembly (wasm-bindgen), Vanilla JS, HTML5, Pico.css.

## 🚀 Comandos Frequentes
- Build do Wasm para Web: `wasm-pack build --target web`
- Rodar testes matemáticos no Rust: `cargo test`
- Limpar artefatos: `cargo clean`

## 📐 Diretrizes Estritas de Código (Rust)
1. **Segurança de Tipos e Erros:** Zero `unwrap()` ou `expect()` no código de produção. Todas as funções expostas ao JS via `#[wasm_bindgen]` devem retornar `Result<T, JsValue>`. Capture divisões por zero ou arrays vazios graciosamente.
2. **Matemática Estatística:**
   - SEMPRE use a correção de Bessel ($n-1$) para variância e desvio padrão.
   - Para amostras independentes, use estritamente o **Teste t de Welch** (não assuma variâncias iguais).
   - Para o teste pareado, valide rigorosamente se as duas matrizes possuem o mesmo tamanho antes de calcular o vetor de diferenças ($d$).
3. **DRY (Don't Repeat Yourself):** Isole o cálculo de médias e variâncias em funções auxiliares privadas.

## 🎨 Diretrizes de Frontend (JS/UI)
1. **Minimalismo:** O HTML deve ser semântico e utilizar as classes padrão do **Pico.css** (via CDN). Não escreva arquivos CSS customizados.
2. **Dinamismo:** O JavaScript deve ouvir um `<select>` de "Tipo de Teste" e exibir/ocultar os campos de input (`textarea`, `input` de $\mu$) de acordo com a necessidade de cada teste.
3. **Integração:** Importe o módulo wasm nativamente via ES Modules.

## 🎯 Objetivo de Ouro
Entregar um MVP funcional, rápido e à prova de balas matematicamente. Se a estrutura base for concluída rapidamente, o objetivo bônus é implementar o cálculo do P-value.
