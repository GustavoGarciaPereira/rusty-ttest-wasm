# 🦀 Rusty T-Test Wasm

Uma calculadora estatística de alta performance rodando inteiramente no navegador. Este projeto implementa os 3 principais Testes-t de Student (One-sample, Independent/Welch e Paired) utilizando um motor matemático em **Rust** compilado para **WebAssembly (Wasm)**, integrado a uma interface ágil em Vanilla JS.

![Screenshot da Interface](https://img.shields.io/badge/UI-Pico.css-blue)
![Rust](https://img.shields.io/badge/Core-Rust_2021-orange)
![WebAssembly](https://img.shields.io/badge/Target-Wasm32-yellow)

## ✨ Principais Funcionalidades

- **Cálculo Estatístico de Alta Precisão:** Suporte a One-Sample, Independent (Welch) e Paired T-tests.
- **P-Value Bicaudal:** Cálculo de probabilidade com precisão acadêmica (utilizando a distribuição t de Student via crate `statrs`) e marcadores de significância (`*`, `**`, `***`).
- **Importação Inteligente de CSV (Drag & Drop):** Arraste planilhas reais para o navegador. O sistema detecta automaticamente o separador (`,` ou `;`) e extrai apenas dados numéricos limpos.
- **Mapeamento Dinâmico de Colunas:** Interface gerada dinamicamente para que o usuário relacione as colunas do CSV com as variáveis do teste.
- **Privacidade Total (Local-first):** Nenhum dado é enviado para servidores. O parsing do CSV e o cálculo matemático ocorrem 100% na memória RAM do client-side.

## 🎯 Por que Rust + WebAssembly?

Aplicações de análise de dados baseadas apenas em JavaScript puro costumam esbarrar em três problemas estruturais que esta arquitetura resolve nativamente:

1. **Precisão Matemática:** O cálculo do P-value exige acesso à Função Distribuição Acumulada (CDF) da distribuição t. O ecossistema JS carece de bibliotecas matemáticas enxutas e precisas para isso. Usando Rust e a crate `statrs`, garantimos resultados equiparáveis ao R ou SciPy.
2. **Type Safety e Tratamento de Erros:** O JavaScript propaga falhas matemáticas silenciosamente (ex: divisões por zero gerando `NaN` ou `Infinity`). A tipagem rígida do Rust e o modelo de `Result<T, E>` nos forçam a tratar casos de borda (como desvio padrão zero) em tempo de compilação, retornando erros claros para a UI.
3. **Isolamento de Responsabilidades:** O JS atua apenas como a "cola" visual (manipulando a DOM e a File API para o CSV), enquanto o Rust assume todo o "heavy lifting" matemático em velocidade quase nativa, sem onerar a thread principal do navegador.

## 🛠️ Casos de Uso

Ideal para testes rápidos de hipóteses no dia a dia, como:
- **Agronegócio:** Comparar ganho de peso em lotes de gado após mudança nutricional (Teste Pareado).
- **Gestão Pública:** Analisar se há diferença estatística significativa nos gastos diários entre duas secretarias independentes (Teste de Welch).
- **Educação/Saúde:** Validar se a média de uma amostra foge de um padrão nacional pré-estabelecido.

## 💻 Como rodar localmente para desenvolvimento

1. Clone o repositório:
   ```bash
   git clone [https://github.com/GustavoGarciaPereira/rusty-ttest-wasm.git](https://github.com/GustavoGarciaPereira/rusty-ttest-wasm.git)
   cd rusty-ttest-wasm


2. Compile o código Rust para WebAssembly:
```bash
wasm-pack build --target web
```

3. Inicie um servidor local (necessário para carregar módulos Wasm sem bloqueios de CORS):
```bash
python -m http.server 8000
```

4. Acesse http://localhost:8000 no seu navegador.

🧪 Testes Unitários
O core matemático foi construído isolando a lógica pura do bindgen do JS. Isso permite rodar a suíte de validação estatística nativamente em x86:
<img width="358" height="886" alt="image" src="https://github.com/user-attachments/assets/ffac4aae-583f-459d-8bd5-8bc285e367a0" />


```bash
cargo test
```
