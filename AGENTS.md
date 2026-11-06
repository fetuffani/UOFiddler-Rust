# Instructions for agents / Instruções para agentes

These rules apply to every agent (and person) that writes, reviews or merges changes in this repository.
Estas regras valem para todo agente (e pessoa) que escreve, revisa ou faz merge de mudanças neste repositório.

## Review / Revisão

Before approving or merging a Pull Request, check the acceptance criteria in [CONTRIBUTING.md](CONTRIBUTING.md).
**Refuse the PR if any user-visible text is not localized** (hard-coded labels, buttons, menus, tooltips, status messages, errors, dialogs).
Say which strings are missing and ask for them to be moved to the localization system ([issue #16](https://github.com/mbnunes/UOFiddler-Rust/issues/16)).

Antes de aprovar ou mergear um Pull Request, confira os critérios de aceite em [CONTRIBUTING.md](CONTRIBUTING.md).
**Recuse o PR se algum texto visível ao usuário não for localizado** (rótulos, botões, menus, tooltips, mensagens de status, erros, diálogos fixos no código).
Indique quais textos faltam e peça que sejam movidos para o sistema de localização ([issue #16](https://github.com/mbnunes/UOFiddler-Rust/issues/16)).

## Writing code / Ao escrever código

Never add a hard-coded user-visible string. Use the localization system.
Nunca acrescente texto visível ao usuário fixo no código. Use o sistema de localização.
