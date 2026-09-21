# ValidationRules quick reference

Path syntax and resolution are owned by [Libertas Schema Path](../reference/schema.md#libertas-schema-path);
examples below apply its validation-expression forms.

Put a rule on the narrowest reusable owner: a named type for every use, a field
or variant for one use, or a function for relationships among its arguments.
Rules inherited from types and fields are cumulative.

Use `@LibertasValidationRules('expression : "MESSAGE_KEY";')` in TypeScript or
`#[libertas_validation_rules(r#"expression : "MESSAGE_KEY";"#)]` in Rust.
Every expression is Boolean, pure, and deterministic. It must be followed by
one `:`, a quoted key declared in the package `StringResources`, and `;`.
The default-locale resource value must be non-empty; clients display its
localized value when the rule fails. Multiple expression/message pairs are
ANDed. Either `FinalOnly=true;` or `FinalOnly=false;` may appear only as the
first directive inside that string. Omission means true, so rules run only during final validation
unless the author explicitly writes `FinalOnly=false;`. There is no separate
`FinalOnly` annotation.

| Purpose | Syntax |
| --- | --- |
| Annotated value / descendant | `[]`, `[field/child]` |
| Function argument | `[argument]` |
| List cursor | `all(item in [items], [item/value] >= 0)` |
| Paths across parent lists | `all(item in [groups/items], [item/value] >= 0)` |
| Filter and project | `collect(node in [nodes], [node] is Active, [node/device])` |
| Collection uniqueness | `is_unique_set(collection)` |
| Reference-context path(s) | `{$.value}`, `{$.preferred,$.fallback}` |
| First non-null value | `[preferred] \| [fallback] \| 0` |
| Logic and comparison | `!`, `&&`, `||`, `==`, `!=`, `<`, `<=`, `>`, `>=` |
| Active Union variant | `[value] is VariantName` |
| Lists and collections | `all`, `any`, `none`, `collect`, `is_unique_set`, `count` |
| String length | `length([name])` |

Square paths start at the annotated value (or a function's synthetic argument
Table). Curly paths read the interaction's
[reference context](../reference/protocols.md#interaction-reference-context) under the
[reference availability contract](../reference/schema.md#availability-null-and-interaction-ownership). Missing descendants, inactive Union fields, invalid indices,
and descendants of null also evaluate to null.

Single `|` evaluates operands from left to right and returns the first non-null
value without changing its type. It does not evaluate later operands after a
winner. `false`, `0`, `""`, empty collections, and objects are non-null and win.
Use the selected value in an explicit comparison when it is not Boolean; the
complete rule must still be Boolean and no general truthiness conversion exists.

Traversal variables are concrete node cursors, never indexes. A traversal
source must select arrays; named path segments automatically fan out across
any parent arrays. In `collect(cursor in source, predicate, projection)`, the
predicate is evaluated first and the projection is resolved only when it is
true. This permits a Union filter such as `[node] is Active` to guard the
variant-specific `[node/device]` projection. The result preserves order and
duplicates; use `is_unique_set(...)` to reject duplicates. Do not emit the
removed index form `[items/i/value]`.

Only rule sets explicitly beginning with `FinalOnly=false;` run after every
proposed node change; failure restores the previous value. Allow incomplete
editing explicitly when opting in, for example
`FinalOnly=false; [] == null || [] >= 0 : "NONNEGATIVE";`. Final validation
runs every rule plus all structural/mandatory schema checks. A null-tolerant
rule never makes a non-nullable field optional. Clients stop at the first
failing applicable rule in declaration order and display its localized resource,
or a generic validation failure if the installed message is missing or blank.

Use precise types and ordinary schema attributes for structural constraints.
If absence is itself invalid, test `!= null`; if a whole rule set has no useful
incremental meaning, keep the conservative default. Use `FinalOnly=false;`
only when immediate editor feedback is intended.

For a Number or DateTime whose limit depends on current data or reference-context data, use
`@LibertasRangeMin("expression")` / `@LibertasRangeMax("expression")` or the
Rust `#[libertas_range_min("expression")]` /
`#[libertas_range_max("expression")]` forms. Each value is one bound expression
compatible with the host's semantic type, without an `expression : "KEY";`
rule pair or `FinalOnly` directive. For an ordinary Number maximum of 100,
use `100`; the Boolean predicate `[] <= 100` is invalid here. DateOnly and
DateTime require expressions of their respective date types. Statically incompatible result types reject the schema; incompatible
non-null runtime values reject validation. Missing reference-context data yields
null and no bound. Follow [dynamic numeric ranges](../reference/validation-rules.md#31-dynamic-numeric-ranges)
for compatible typed dates, numeric limits, and GUI/final enforcement.

Treat every intermediate value as semantically typed. DateOnly plus/minus an
integer shifts Gregorian calendar days; DateTime plus/minus an integer shifts
epoch seconds. Subtracting two DateOnly values returns elapsed days, and
subtracting two DateTime values returns elapsed seconds. Never perform numeric
arithmetic on DateOnly's `yyyyMMdd` wire digits.

Use the normative [rule language](../reference/validation-rules.md)
for grammar, typing, null semantics, failure reporting, and edge cases.
