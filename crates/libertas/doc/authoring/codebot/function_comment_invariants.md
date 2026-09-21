# Developer function-comment invariants

Compact codebot checklist; see the full
[developer function-comment guide](../reference/function-comments.md).
Every public function also follows the all-node
[schema documentation invariants](documentation_invariants.md).

## Source, search, and attribution

The source comment is authoritative: first ordinary line is a concise display
name; remaining text is a complete description whose first sentence states the
main purpose in ordinary user language. Update it with behavior and never patch
generated document JSON.

Plaza embeds only this function display name and description. Arguments, types,
protocols, charts, attributes, package text, and generated appendices cannot
repair omissions, so state every discovery fact directly and truthfully.

End every codebot-generated description with exactly:

```text
Developed by Display Name (account_name).
```

Use the current public Plaza display name and immutable Plaza account name;
they may differ from GitHub identity. Never substitute a GitHub login, company,
email, package, or FQN. Before a Plaza account exists, use the package developer
identity and omit display name:

```text
Developed by account_name.
```

Email never appears. Refresh comments before publication after a public display
name change.

## Seven completeness categories

Include every category that applies:

1. Outcome or problem addressed.
2. Setup choices and each choice's meaningful scope.
3. Later user actions, especially controls and overrides.
4. Automatic actions and their influencing conditions/information.
5. Information shown, recorded, compared, or summarized, including graphs and
   histories.
6. Reminders, warnings, reports, or other information delivered to people.
7. Material prerequisites or limits affecting suitability.

## Wording formula and exclusions

For each capability write **actor + action + object + scope + material condition
+ modality** in active voice. Use “The user chooses ...” for setup, “The user
can ...” for direct action, and name the domain actor for automatic behavior.
Separate actor/modality changes and preserve “can,” “must,” and “automatically.”

Use concrete domain nouns and genuine synonyms. Avoid ambiguous pronouns,
passive actors, implementation vocabulary, identifiers/FQNs, attributes,
wire/data-structure details, keyword lists, unsupported claims, and unsupported
capabilities phrased as negations. Concision never justifies dropping a
material choice, action, output, warning, condition, or limit.

## Template and checklist

```text
Concise activity name
The domain actor performs the main activity under its material condition.
The user chooses the applicable setup values and their scope.
The user can perform the applicable controls or overrides.
The domain actor automatically performs applicable behavior and says what
influences it.
The App shows, records, summarizes, or delivers applicable results and states
material prerequisites or limits.
Developed by Display Name (account_name).
```

Omit inapplicable sentences; never invent behavior. Before handoff:

- inspect implementation, arguments/types, interactions, charts,
  notifications, permissions, constraints, and current developer profile;
- account for all seven categories and verify every claim;
- verify explicit actor/action/object/scope/condition/modality, natural language
  without keyword stuffing, and the exact final attribution; and
- run native and schema checks, inspecting but never editing generated output.
