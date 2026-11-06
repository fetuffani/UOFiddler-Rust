# Contributing / Como contribuir

## Acceptance criteria for every Pull Request / Critérios de aceite de todo PR

A Pull Request that fails any item below **must be rejected** by reviewers and by agents (human or automated) until it is fixed.
Um PR que não cumpra qualquer item abaixo **deve ser recusado** por revisores e por agentes (pessoas ou automatizados) até ser corrigido.

### 1. All user-visible text is localized / Todo texto visível ao usuário é localizado

Every text the user can see must go through the localization system (tracked in [issue #16](https://github.com/mbnunes/UOFiddler-Rust/issues/16)), never hard-coded in the code:

- labels, buttons, menus, tabs, tooltips and placeholders;
- status bar messages, errors, warnings and dialogs;
- any other string shown in the interface.

Adding or changing a user-visible string without localizing it is a reason to refuse the PR. New strings must come with a translation key and at least the default (English) text; the other supported languages follow the fallback rules of the localization system.

Todo texto que o usuário pode ver deve passar pelo sistema de localização (acompanhado na [issue #16](https://github.com/mbnunes/UOFiddler-Rust/issues/16)), nunca fixo no código:

- rótulos, botões, menus, abas, tooltips e textos de exemplo (placeholders);
- mensagens da barra de status, erros, avisos e diálogos;
- qualquer outra string exibida na interface.

Acrescentar ou alterar texto visível sem localizá-lo é motivo para recusar o PR. Textos novos devem vir com a chave de tradução e ao menos o texto padrão (inglês); os demais idiomas seguem a regra de fallback do sistema de localização.

### How to localize / Como localizar

- Use `t!("key")` (or `t!("key", n = 3)` for `{n}` parameters) from `crates/uo_gui/src/i18n.rs`; never a string literal in the UI code.
- Add the key to **every** file in `crates/uo_gui/locales/` (`en.txt` is the default and the fallback). The tests fail if a language has missing or extra keys, or different `{parameters}`.
- To add a language: create `locales/<code>.txt` with all the keys and list it in `LANGUAGES` in `i18n.rs`.
- The test `hardcoded_ui_text_never_grows` counts the legacy hard-coded texts per file (`crates/uo_gui/i18n_baseline.txt`) and fails when one grows. After migrating texts, lower the counts with `UPDATE_I18N_BASELINE=1 cargo test -p uo_gui hardcoded`.

- Use `t!("chave")` (ou `t!("chave", n = 3)` para parâmetros `{n}`) de `crates/uo_gui/src/i18n.rs`; nunca uma string literal no código da interface.
- Acrescente a chave em **todos** os arquivos de `crates/uo_gui/locales/` (`en.txt` é o padrão e o fallback). Os testes falham se um idioma tiver chaves faltando ou sobrando, ou `{parâmetros}` diferentes.
- Para adicionar um idioma: crie `locales/<código>.txt` com todas as chaves e liste em `LANGUAGES` no `i18n.rs`.
- O teste `hardcoded_ui_text_never_grows` conta os textos fixos legados por arquivo (`crates/uo_gui/i18n_baseline.txt`) e falha se algum crescer. Depois de migrar textos, reduza os números com `UPDATE_I18N_BASELINE=1 cargo test -p uo_gui hardcoded`.

### Other criteria / Outros critérios

More criteria can be added to this list. / Novos critérios podem ser acrescentados a esta lista.

## Git LFS

Binary assets (images, game data files, archives, executables) are stored with [Git LFS](https://git-lfs.com) (see `.gitattributes`). Install it once with `git lfs install` before committing such files; the CI checkout does not download LFS files yet (the Linux job runs in a container without `git-lfs`), so nothing the build needs may be stored in LFS until the workflow installs `git-lfs` and sets `lfs: true`. Files that were committed before LFS was enabled (for example `packaging/linux/uofiddler.png`) stay as regular files.

Arquivos binários (imagens, arquivos de dados do jogo, pacotes, executáveis) ficam no [Git LFS](https://git-lfs.com) (veja `.gitattributes`). Instale uma vez com `git lfs install` antes de commitar esses arquivos; o checkout do CI ainda não baixa arquivos LFS (o job Linux roda num container sem `git-lfs`), então nada que o build precise pode ficar no LFS até o workflow instalar o `git-lfs` e usar `lfs: true`. Arquivos commitados antes de ativar o LFS (por exemplo `packaging/linux/uofiddler.png`) continuam como arquivos normais.

## Process / Processo

- One PR per issue, referencing it (`Closes #N` only when the issue is fully solved; otherwise `Refs #N`).
- The build must pass on Windows and Linux (GitHub Actions) before merging.
- Prefer small commits with a clear message.
