# Schema documentation invariants

Compact codebot checklist; see the full
[schema documentation contract](../reference/documentation.md) for
rationale, examples, output structure, and edge cases.

## Coverage and wording

Source comments are authoritative; never patch generated schema/document JSON.
Give every authored schema-visible node a short natural label on its first
ordinary line and a complete plain-language description on subsequent ordinary
lines. A use-site and its referenced named type each need independent text.

| Authored node | Required meaning |
| --- | --- |
| Package | Recognizable capability name; outcome, audience/setting, and material prerequisites or limits. |
| Function | Activity name; complete purpose, choices, later actions, automatic behavior, results, and limits under the [function-comment invariants](function_comment_invariants.md). |
| Named type | Real-world concept represented by one value and why a person supplies or sees it. |
| Parameter or input field | What the person chooses/enters, with effect and scope. |
| Output or read-only field | What the App reports, records, or calculates and when it matters. |
| Boolean or nullable value | Positive state/choice; material true/false consequences; how absence differs from zero, false, empty, or unavailable. |
| Number, date, time, duration, physical quantity, or free text | Domain meaning, scope, and suitable content/use; keep encoding, unit, and format in metadata. |
| Libertas system object | Domain role such as patient, rain gauge, vehicle, or valve; retain the typed object rather than documenting an ID. |
| Enumeration/bitflags type and every member | Whole decision/state/option set; standalone member label and consequence, including zero, `None`, and `Unknown`. |
| Union and every variant | Whole action/event/state/result family; standalone variant trigger/scope and consequence, never a role, discriminator, or version token. |
| Inline value or field | The same complete text as its named equivalent. |
| List container and item | Above `----`: plural label and collection purpose/order/limits; below: singular label and one entry's meaning/use. |
| Hidden, interaction-context, protocol, persistent-data, or chart value | Truthful domain purpose without a secret or implementation mechanism; none is exempt. |
| Custom-property or `StringResources` value | Natural, complete, translatable user text under these same rules. |

The only ordinary exception is a parser-suppressed compiler-generated
TypeScript discriminator. Codebots must not use positional `LibertasEnum`
catalogs because their fabricated members cannot carry required documentation.

- Author and translate natural, idiomatic, technology-neutral user language
  under the [user-language invariant](../reference/documentation.md#universal-user-language-invariant).
  Describe an App's “endpoint” as its service or specific capability, using the
  natural target-language equivalent; retain precise domain terms when needed.
  Preserve all material meaning and every identifier or message placeholder.
  Labels are concise, standalone, sibling-distinguishable, and readable aloud;
  descriptions add purpose and, for behavior, name actor, action, object, scope,
  material condition, and modality in active voice.
- Explain material empty, unavailable, zero, false, `None`, or `Unknown`
  meanings. Do not rely on order, position, color, shape, icons, animation, or a
  client-specific gesture.
- Include relevant [domain references and alternate names](../reference/documentation.md#domain-references-and-discoverability)
  in natural descriptions: formal references, form/document numbers,
  abbreviations, and common terminology, with their actual supported scope.
  Document each bound topic or choice as well as a shared link operation;
  an Enumeration destination owns this vocabulary in its member description.
- Match text to its use: inputs explain entry and effects, outputs explain
  reported meaning and availability, and internal schema records explain domain
  purpose. For blank String answers say “Leave it empty if absent” and describe
  any fallback or grouping effect. Keep AI-import instructions consistent with
  [the field's blank semantics](../reference/documentation.md#parameter-and-field).
- Do not expose internal identifiers/casing, codes, FQNs, Libertas Schema Paths, byte/storage/wire
  mechanics, or implementation terms such as field, type, struct, table, enum,
  Union, variant, schema, payload, protocol, endpoint, request, response,
  runtime, discriminator, or index.
- Visible chart text uses an Enumeration for closed vocabulary,
  `FormattedText` for dynamic App wording, or a Libertas system-object type for
  identity. Plain `String` is only real literal domain/user-authored text.
- Do not duplicate platform-generated [AI tool-call representation hints](../reference/presentation.md#ai-tool-call-descriptions)
  in comments; declare the appropriate attributes and retain domain-specific
  meaning, scope, consequences, and limits.

## Exact source and property syntax

- TypeScript package text uses leading `///` in the selected entry module and
  declarations use JSDoc. Rust package text uses leading `//!` in `src/lib.rs`;
  declarations and typed parameters use `///`. Ordinary `//` is not localized.
- A List divider is a line of at least four hyphens. Text/properties above it
  belong to the container; those below belong to the item.
- Put ordinary text and property blocks before the first TypeScript
  `@Libertas...` tag; extraction stops there.
- Write long descriptions as HTML-style flowing prose. One logical CR/LF break
  is source wrapping and becomes exactly one space; two or more logical breaks
  become one LF paragraph break. CRLF counts as one break. Wrap adjacent source
  lines freely, and leave a blank documentation line only for an intentional
  paragraph break. This applies to `Description`, not property or
  `StringResources` values.

Use a case-sensitive property marker alone on its line after ordinary text:

```text
[PropertyName]
Value through the line before the next marker or section end.
```

Do not repeat a property in one section. Outer blank space is trimmed; internal
newlines remain. Names/structural keys are stable; values are localized and
flattened on the owner. Unknown valid properties are preserved, but have no
generic Libertas behavior; use only a documented consumer property.
`DisplayName`, `Description`, `List`, `StringResources`, `Strings`, `Enum`, and
`DocumentVersion` are globally reserved; any structural key at that owner,
including `Locale`, `Types`, `Name`, or `Type`, is also reserved.

### Standard properties

<!-- LIBERTAS_DOCUMENT_EXTENSION_INVENTORY_START -->
- `[DefaultText]`: initial task name on a function, or initial text on an effective
  `String` or `EndpointServer` field; one non-empty localized line of safe content.
  It emits no schema `Default` and never replaces persisted/user-assigned text,
  including a saved empty string.
- `[TrueText]` and `[FalseText]`: optional, non-empty single-line labels on Boolean
  fields. Either selects explicit choices; only non-nullable Booleans with an
  explicit Default and no custom labels may use a switch.
- `[NullText]`: optional, non-empty single-line caption for explicit null on a
  nullable field, including its clearing choice; omission uses localized None.
<!-- LIBERTAS_DOCUMENT_EXTENSION_INVENTORY_END -->

These properties never replace ordinary labels/descriptions. Omit a property
whose intended text exactly matches its ordinary localized fallback. Initial
names remain concise, editable and truthful; never invent personal details.

## StringResources prose

One package-owned `StringResources` table serves every function/type; keys are
stable and untranslated, while values are localized complete natural LMF1
messages, never concatenated fragments. Required permission/`AccessHost`
reasons use exact same keys and explain access, App action, and user benefit.
Errors say what happened and possible recovery; notifications/status name
actor, object, scope, and condition without diagnostics.

A human-visible data/database name is one canonical `DataName` byte blob: a
stable package key plus ordered typed Libertas Message Arguments, never
rendered, literal, Base64, or separately maintained machine text. Translation
preserves every LMF1 position, type, style, selector, plural offset, brace, and
apostrophe escape; objects and Enumerations remain typed arguments.

## Checklist

- Inventory every node above, both List halves, and every member/variant/inline
  value; synchronize labels/descriptions with behavior.
- Keep engineering notes outside localized documentation; verify property
  ownership, resource keys, reasons, and messages.
- Verify defaults are useful, truthful, and differ from ordinary fallbacks;
  optional null captions describe explicit null without changing its value.
- Inspect the generated default document; run native checking and Studio
  validation; confirm wrapped descriptions contain spaces while intentional
  paragraphs contain one LF; fix reported source and never generated JSON.
