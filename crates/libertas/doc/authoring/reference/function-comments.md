# Developer function-comment invariants

This document is the normative authoring contract for a codebot that creates or
changes a public Libertas App function. The function's source documentation is
its primary human-authored discovery description. It must stand on its own;
supporting text derived from types and attributes cannot repair an incomplete or
ambiguous function comment.

It specializes the all-node
[schema documentation contract](documentation.md). A public
function must satisfy both contracts; this page adds discovery completeness and
developer attribution rather than relaxing the ordinary documentation rules.

Plaza calculates exactly one discovery embedding for each public function.
Its only natural-language input is the function comment's display name and
description. Arguments, types, protocols, charts, attributes, package text, and
generated appendices are not added. Consequently, every fact needed to discover
the function must appear naturally and truthfully in this comment.

These are code-authoring and review invariants. They do not imply that the
language parser or Studio can judge the quality of natural-language prose.

## Source ownership and extraction

- The source comment is authoritative. Never edit generated schema or localized
  document JSON to change it.
- Every public function must have a non-empty documentation comment.
- The first ordinary text line is the display name. It must be a concise name,
  not a sentence or keyword list.
- The remaining ordinary text is the description. Its first sentence must state
  the function's main purpose in terms an ordinary user understands.
- Rust uses outer rustdoc (`///`) on the function. TypeScript uses a JSDoc block
  (`/** ... */`) on the function. Language-specific attributes keep their
  required syntax but are not a substitute for the description.
- When behavior changes, the source comment must change in the same patch.

## Developer attribution invariant

Every codebot-generated public-function description must end with a developer
attribution so both developer names participate in the function's search text.
Use this exact natural-language shape when the developer has a Plaza account:

```text
Developed by Display Name (account_name).
```

`account_name` is the immutable public Plaza account name and `Display Name` is
the current public profile display name. A codebot must obtain both from the
developer profile and must not substitute a GitHub login, company name, email,
package name, or fully qualified name.

Before the developer has a Plaza account, the display name may be omitted. Use
the developer identity declared by the package:

```text
Developed by account_name.
```

The attribution is ordinary description text, not a custom document property.
If the public display name changes, regenerate the affected function comments
before the next publication. Email never belongs in a function comment or
search document.

## Completeness invariant

The description must include every user-relevant capability declared or
implemented by the function. Include each applicable category and omit
categories that do not apply:

1. The outcome or problem the function addresses.
2. Choices the user makes during setup, including the meaningful scope of each
   choice.
3. Actions the user can take after setup, especially manual controls and
   overrides.
4. Actions performed automatically and the conditions or information that
   influence them.
5. Information the function shows, records, compares, or summarizes, including
   meaningful graphs and histories.
6. Reminders, warnings, reports, or other information delivered to people.
7. Important prerequisites or limits that affect whether the function is
   suitable for the user's need.

The function comment must express these capabilities directly. A codebot must
not assume that a reader or a search model will reconstruct them from argument
names, referenced types, attributes, or generated appendices.

## Actor, action, and scope invariant

Use active voice and name the actor for every capability:

- Use **“The user chooses ...”** for setup choices.
- Use **“The user can ...”** for direct actions and later adjustments.
- Name the application or domain actor for automatic behavior, such as
  **“The sprinkler adjusts ...”** or **“The monitor warns ...”**.
- State the action's object and scope, such as **“for each zone”**, **“for the
  selected patient”**, or **“across all study samples”**.
- State a material condition when it distinguishes the capability, such as
  **“based on recent rain and forecast weather”**.
- Use a separate sentence when the actor, capability, or modality changes.

Do not use passive constructions such as “chosen by the user” when active voice
is available. Do not use an unclear pronoun such as “it” where a reader or model
could confuse a user action with automatic behavior. Preserve meaningful
modality: “can,” “must,” and “automatically” are not interchangeable.

## Language invariant

Write for people with no software background while retaining the vocabulary of
the actual domain, whether the App is for a home, business, laboratory,
engineering facility, or healthcare setting.

- Describe user goals and observable behavior, not internal architecture.
- Use concrete domain nouns and verbs. Prefer “shows water use by day” over
  “provides data.”
- Include natural synonyms only when they are names people genuinely use for
  the capability. Do not append keywords or repeat a sentence to influence
  search.
- Do not mention a capability that the function does not provide.
- Do not list unrelated unsupported capabilities as negations. A semantic match
  may still associate the function with the negated words.
- Do not expose source identifiers, fully qualified names, attributes, wire
  details, or data-structure terminology in user-facing prose.
- Keep the description concise, but never shorten it by dropping a material
  user choice, direct control, automatic behavior, result, or limit.

## Example

This wording is ambiguous and incomplete:

> **Smart Sprinkler**
>
> Manages irrigation using settings chosen by the user. It adjusts watering
> based on weather and supports manual adjustment.

It does not say who performs the adjustment, what can be adjusted, or its
scope. Use explicit actors and actions instead:

> **Smart Sprinkler**
>
> The sprinkler automatically waters each lawn, garden, or planted area
> according to its plants, recent rain, and forecast weather. The user chooses
> each area's zone valve, plant type, watering-head type, preferred watering
> periods, and watering pauses. The user chooses reminder recipients. The user
> can manually change the watering percentage for each zone at any time. The
> sprinkler shows current status and upcoming watering plans, provides
> water-balance, water-use, and weather graphs, and sends reminders before
> freezing weather. Developed by Libertas (libertas).

The following fragments demonstrate only the function comment; `/* arguments */`
is not copyable schema source. In a complete App, document every real parameter,
referenced type, and field under the
[schema documentation contract](documentation.md).

In Rust, place that text directly above the function:

```rust
/// Smart Sprinkler
/// The sprinkler automatically waters each lawn, garden, or planted area
/// according to its plants, recent rain, and forecast weather.
/// The user chooses each area's zone valve, plant type, watering-head type,
/// preferred watering periods, and watering pauses.
/// The user chooses reminder recipients.
/// The user can manually change the watering percentage for each zone at any time.
/// The sprinkler shows current status and upcoming watering plans, provides
/// water-balance, water-use, and weather graphs, and sends reminders before
/// freezing weather.
/// Developed by Libertas (libertas).
pub fn smart_sprinkler(/* arguments */) {
    // implementation
}
```

In TypeScript, use the equivalent JSDoc placement:

```typescript
/**
 * Smart Sprinkler
 * The sprinkler automatically waters each lawn, garden, or planted area
 * according to its plants, recent rain, and forecast weather.
 * The user chooses each area's zone valve, plant type, watering-head type,
 * preferred watering periods, and watering pauses.
 * The user chooses reminder recipients.
 * The user can manually change the watering percentage for each zone at any time.
 * The sprinkler shows current status and upcoming watering plans, provides
 * water-balance, water-use, and weather graphs, and sends reminders before
 * freezing weather.
 * Developed by Libertas (libertas).
 */
export function smartSprinkler(/* arguments */): void {
    // implementation
}
```

## Codebot workflow

Before writing or updating a function comment, a codebot must:

1. Inspect the function implementation, arguments, referenced types, user
   interactions, graphs, notifications, permissions, relevant constraints, and
   current developer profile.
2. Make a private inventory using the seven completeness categories above.
3. Draft one coherent description in active voice, starting with the main
   purpose and naming the actor for every capability.
4. Verify every statement against source or validated schema evidence. Do not
   invent behavior from a name or sample.
5. Compare the final comment with the inventory and restore any material fact
   lost during editing.
6. Append the required developer attribution using the current Plaza account
   and display names, or the package developer identity before an account
   exists.
7. Run the normal language and schema checks. Treat a generated document as an
   output to inspect, never as the source to patch.

## Review checklist

- The display name and purpose are clear without reading code.
- Every applicable completeness category is represented.
- Every user choice and direct action names the user.
- Every automatic action names the responsible domain actor.
- Objects, scopes, conditions, and modality are explicit.
- The description uses ordinary, truthful domain language without keyword
  stuffing, implementation vocabulary, or capability negation.
- The source comment and current behavior agree.
- The final description sentence contains the required developer attribution;
  it contains no email address.
- A translation can preserve every actor, action, scope, condition, and
  modality without guessing.

## Related contracts

- [App authoring contract](app-contract.md) defines the general function, schema, and
  localization rules.
- [Schema documentation contract](documentation.md) defines the
  user-language contract for every schema-visible document node and extension.
- [Presentation and localization](presentation.md) defines documentation
  extraction, formatted text, value display, and chart authoring.
- [Rust manual](languages/rust.md) and [TypeScript manual](languages/typescript.md) define the source
  syntax available to a codebot.
- [Schema document translation guide](../guides/translation.md)
  defines how localized documents preserve meaning and structure.
- [Plaza publication and discovery contract](publication.md) defines how
  function descriptions participate in discovery.
