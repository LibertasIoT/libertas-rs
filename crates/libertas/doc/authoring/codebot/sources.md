# Source policy

Use the installed public SDK for runtime APIs and the
[shared public references](../reference/index.md) for detailed authoring
contracts. The [App contract](../reference/app-contract.md) identifies source
language and shared schema responsibilities. CodeBot cards are compact task
context; examples do not define additional APIs or override those owners.

If a required public contract is missing or contradictory, report the exact
symbol, package/SDK version, conflicting sections, and desired behavior to the
requester. Use the project's published issue/support channel when one is
available and the task authorizes reporting; do not invent a private contact or
inspect private implementation to fill an App-contract gap.

## Public SDK documentation location

For prompts and tools that need a public URL, the Rust SDK bundle's entry point is
[Libertas CodeBot guidance](https://github.com/LibertasIoT/libertas-rs/blob/main/crates/libertas/CODEBOT.md).
Its [complete documentation tree](https://github.com/LibertasIoT/libertas-rs/tree/main/crates/libertas/doc/authoring)
contains the linked contracts and guides. Prefer the installed SDK's local
`CODEBOT.md` for version-matched work; `main` represents the latest pushed bundle.
A first-time bundle becomes available only after the SDK commit is pushed.
