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

### Other criteria / Outros critérios

More criteria can be added to this list. / Novos critérios podem ser acrescentados a esta lista.

## Process / Processo

- One PR per issue, referencing it (`Closes #N` only when the issue is fully solved; otherwise `Refs #N`).
- The build must pass on Windows and Linux (GitHub Actions) before merging.
- Prefer small commits with a clear message.
