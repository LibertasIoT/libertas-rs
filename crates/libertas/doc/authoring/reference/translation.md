# Translation document contract

- Read and write UTF-8 JSON without Markdown fences or explanatory output.
- Set `Locale` to the locale tag used in the output filename.
- Locale tags are 1–128 ASCII alphanumeric characters grouped into segments
  separated by one `-` or `_`; leading, trailing, or repeated separators are
  invalid.
- Preserve `DocumentVersion`, object structure, arrays, ordering, numeric values,
  and structural identifiers.
- Do not translate structural keys, native type/function names, string-resource
  keys, or schema IDs.
- Translate package/type `DisplayName` and `Description`, field and enum `Name`
  and `Description`, package `StringResources` values, and custom
  document-property string values. This includes `[DefaultText]` on fields and
  functions, Boolean `[TrueText]`/`[FalseText]`, and nullable-field `[NullText]`.
  Preserve their keys and translate their values as non-empty single lines.
- Apply the [user-language invariant](documentation.md#universal-user-language-invariant)
  to translated prose: use natural, idiomatic, technology-neutral wording while
  preserving every material meaning.
- Keep every translated `Description` in canonical HTML-style flow: one logical
  CR/LF break becomes exactly one space, while two or more logical breaks become
  one LF paragraph break. CRLF counts as one break. This rule does not apply to
  custom-property or `StringResources` values.
- Preserve intentionally empty localized strings.
- Package `StringResources` values use Libertas Message Format 1 (LMF1), based on ICU
  MessageFormat 1 with Libertas types. Argument identifiers are zero-based
  positions, never printf placeholders or names. Preserve every position, type,
  style, selector key, plural offset, brace, and apostrophe escape exactly,
  including `LibertasDevice` and the Enumeration type-name style of
  `LibertasEnum`. Translate only literal message text, including literal text
  inside variants.
- Do not add, remove, reorder, or duplicate schema nodes.

## Migration guarantees

When both signatures are known, functions match by stable native name and types
with the same native name are disambiguated by namespace identity. Fields and
enum entries match by their complete native declaration path. New localized values
receive default-locale text with an explicit review warning; obsolete entries
are removed; translated custom attributes and string resources are retained;
and regenerated numeric IDs are installed.

When an old signature is unavailable, Studio preserves field-level translation
only if the complete numeric shape is identical. If the shape changed, it copies
the new default fields instead of risking a translation being attached to the
wrong field.


For workspace preparation, locale selection, import, and removal, follow the
[translation guide](../guides/translation.md).
