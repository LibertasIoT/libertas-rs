# Libertas Data Validation Rules Specification

**Version:** 1.8

This document is normative for the `ValidationRules` schema attribute,
including client evaluation and presentation of each rule's supplied message.

## 1. Purpose

Libertas Data Validation Rules define a small, statically analyzable language for expressing semantic validity constraints over schema-compliant data.

The input to validation is:

1. a schema \(\Sigma\),
2. one concrete or partially entered `data` instance, and
3. optional `referencedData`: the reference context supplied to evaluation,
   containing its ordered reference objects, or `null` when no context is available.

A rule is evaluated as `Eval(rule, data, referencedData)`. The schema supplies
the rule and type/path interpretation. Under the
[shared reference context](schema.md#context-and-syntax), ordinary curly
references read the [current interaction reference context](protocols.md#interaction-reference-context).
`referencedData` carries that lookup environment without changing this
evaluation signature. External references follow its source order and the
[shared first-non-null lookup rule](schema.md#availability-null-and-interaction-ownership).
It does not imply a fetch, I/O, stored history, or a merged reference object.

The rule language does not define GUI presentation, behavior, state transitions, mutation, or effects.

A rule is a pure expression of `data` and `referencedData`. During editing, any field
MAY temporarily be undefined, including a field that is not declared
`Nullable`. Undefined is local editor state, not an encodable null; the rule
evaluator resolves it as null. Consequently, `== null` and `!= null` are valid tests for every
field. Mandatory-field validation and `ValidationRules` are complementary,
independent requirements. A rule MAY accept a temporary `null`, but that success
does not make the field optional or suppress the final mandatory-field check.
Final submission separately requires structural schema validity, including
every non-`Nullable` field, and all validation rules to pass. A zero-length
[MIME-aware Bytes](schema.md#mime-aware-bytes) value is a present, encodable
Bytes field but has undefined media meaning and evaluates as null. Its
non-`Nullable` declaration and size bounds do not require media selection;
an App requiring a file declares a rule such as
`[] != null : "IMAGE_REQUIRED";`. A defined LMB1 value evaluates as non-null,
even when its content is empty. Ordinary Bytes keep their existing valuation.

ValidationRules uses the [shared schema reference contract](schema.md#availability-null-and-interaction-ownership).
Rules may occur in editable Request/SubscriptionRequest data, inherited types,
list items, and hidden/read-only fields. External reference reads require neither
CopyFrom nor request-side copies of server metadata.

Authors MUST account for first-time request composition with no reference context:
every curly reference evaluates to `null` in that state.
ValidationRules must handle that absence explicitly, for example with a null
guard or a typed `|` fallback. The client still evaluates the rule normally;
absence does not automatically make the rule pass. Neither validation nor
request composition fetches server data to resolve these references.

For the supplied `referencedData`, the data instance is valid iff every
validation rule evaluates to `true`.

\[
Valid_\Sigma(data,referencedData) \iff \bigwedge_{rule\in R} Bool(Eval(rule,data,referencedData))
\]

---

## 2. Data Model

A schema MAY contain:

- primitive values
  - boolean
  - integer
  - floating-point number
  - string
  - DateOnly calendar date
  - DateTime instant
  - other schema-defined primitive types
- structs
- arrays
- nullable values
- union types

Example:

```c
struct A {
    int value;
}

struct B {
    string name;
}

struct Root {
    A | B item;
    float? limit;
    Entry[] entries;
}
```

Structural schema compliance is outside the scope of the validation-rule
language. An evaluator MUST nevertheless tolerate the transient `null` values
used by client editors and MUST remain total for that incomplete state.

Every successful intermediate evaluation carries an internal semantic type;
operators MUST NOT infer a new type merely from the scalar's storage encoding.
In particular, DateOnly is a calendar type even though its schema and wire
encoding is the integer `yyyyMMdd`, and DateTime is an instant type even though
its schema and wire encoding is integral Unix epoch seconds. Literals, paths,
fallback results, operator results, and function results all retain or produce
an explicit internal type. `null` has its own type.

---

## 3. Rule Set

Rules are stored in one string-valued schema attribute:

```json
{ "Name": "ValidationRules", "Value": "[min] <= [max] : \"INVALID_RANGE\";" }
```

`ValidationRules` MAY be declared on:

- a schema function, where the evaluation root is a synthetic Table containing
  all function arguments under their schema names;
- any schema data field; or
- any schema data type, including a named primitive, Table, List, Union, or
  other named Libertas type.

A field/type rule is rooted at each concrete value of the annotated node. A
named-type rule and a field-local rule are cumulative when a field references
that named type. Inherited field rules are cumulative as well. The empty data
path `[]` denotes the annotated value itself. Function rules address arguments
by name, such as `[minimum]` and `[options/timeout]`.

The TypeScript source spelling is
`@LibertasValidationRules("...")`. The Rust source spelling is
`#[libertas_validation_rules("...")]`. A Rust named type may use the
standalone attribute; fields and variants use the `LibertasExport` helper
attribute. Parsers emit the raw string and source location. Libertas Studio's
shared schema validator owns grammar and schema-decidable semantic validation.

A validation rule set is a sequence of expression/message pairs. Every Boolean
expression is followed by one `:`, a quoted package `StringResources` key, and
the terminating `;`:

```text
[min] <= [max] : "INVALID_RANGE";
```

The key is metadata for that complete expression and does not participate in
Boolean evaluation. It MUST be declared by the package `StringResources`
catalog and have a non-empty value in the default-locale document. Studio
enforces both requirements. A bare `expression;` is invalid.

A rule set MAY begin with this case-sensitive phase directive:

```text
FinalOnly=true;
```

or explicitly:

```text
FinalOnly=false;
```

The default is `true`. The directive, when present, MUST be the first item,
MUST occur only once, and accepts only the lowercase literals `true` and
`false`. It is metadata inside the `ValidationRules` string; it is not an
individual schema attribute.

`FinalOnly=false` opts the rule set into both incremental editor validation
and final validation. An omitted directive or `FinalOnly=true` omits the
entire rule set from incremental validation and evaluates it only during final validation. This
removes the need for that rule set to tolerate an initial incomplete editor
state. It does not alter `null` expression semantics, change `Nullable`, or
replace final static schema checks. When field, inherited, and named-type rule
sets accumulate, each declaration retains its own `FinalOnly` value.

### Authoring guidance

Use the schema's precise type and ordinary structural attributes for
nullability, numeric ranges, sizes, and uniqueness. `ValidationRules` express
semantic relationships that those declarations cannot express; they do not
restate or replace structural constraints.

When absence is itself semantically invalid, test it directly, for example
`[approver] != null : "APPROVER_REQUIRED";`. When an entire rule set has no useful incremental meaning, retain the conservative
final-only default. Use leading `FinalOnly=false;` only when the rule should
participate in incremental editor validation.

Example:

```c
[min] <= [max] : "INVALID_RANGE";

[value] >= [min] &&
[value] <= [max] : "VALUE_OUT_OF_RANGE";

[name] != null : "NAME_REQUIRED";
```

All rules are combined using logical AND.

Therefore:

```c
r1 : "ERROR_1";
r2 : "ERROR_2";
r3 : "ERROR_3";
```

is semantically equivalent to:

```c
r1 && r2 && r3 : "COMBINED_ERROR";
```

A data instance is valid only when every rule evaluates to `true`. The two
forms above are validity-equivalent but intentionally not presentation-
equivalent: the first form can display the key belonging to the selected
failed rule, while the combined expression owns one message key.

## 3.1 Dynamic numeric ranges

`RangeMin` and `RangeMax` reuse the Data Validation expression language to
derive live inclusive constraints for a Number or DateTime value. Their source spellings
are `@LibertasRangeMin("expression")` / `@LibertasRangeMax("expression")` in
TypeScript and `#[libertas_range_min("expression")]` /
`#[libertas_range_max("expression")]` in Rust. The language-neutral schema
stores the decoded expression string under `RangeMin` or `RangeMax`.

Each attribute contains exactly one value expression. It MUST NOT contain a
validation message pair (`: "KEY";`), multiple expressions, or a `FinalOnly`
directive. The expression MUST produce a bound compatible with the annotated
node's semantic type, not a Boolean validation predicate. Studio MUST validate
its grammar, paths, calls, operand types, and result type against that node. A
result type known to be incompatible MUST be rejected during schema validation;
unknown reference types remain subject to runtime validation. Nullability does
not make a known incompatible non-null result type valid.

Evaluation uses the same field-local context as `ValidationRules`: `[]` is the
annotated Number or DateTime and square paths traverse from it. Curly paths
inspect the [current interaction reference context](protocols.md#interaction-reference-context),
including in editable requests. The result type must match the control's semantic type:

| Annotated control | Required non-null result |
| --- | --- |
| Ordinary Number, including TimeInterval | Finite Number; integer and fractional bounds are permitted. |
| DateOnly | DateOnly calendar date. |
| DateTime | DateTime instant. |

A numeric literal or an ordinary Number path is not a valid bound for a date
control, even when its digits match that date's wire representation. A
DateOnly result becomes its valid `yyyyMMdd` boundary value and a DateTime
result becomes integral epoch seconds only at the range boundary. Expressions
retain their semantic types.

`null` means that no dynamic bound is currently available, including when the
first request is composed before any reference context exists. It adds no constraint.
A non-null Boolean, String, array, collection, object, mismatched semantic
type, or non-finite numeric result is an error. When an expression's result
cannot be proved compatible statically, clients MUST check it at runtime and reject
validation on an incompatible result. They MUST NOT silently remove the bound,
erase its semantic type, or coerce the result to a Number.

For example, `@LibertasRangeMax('[] <= 100')` is rejected because its result is
Boolean. Use `@LibertasRangeMax('100')` for a numeric maximum, or put the
Boolean predicate in `ValidationRules` with its required message key. Paths
and arithmetic expressions that preserve the required type are supported; bounds are not restricted to literal constants. For example, a
DateTime path plus an integer number of seconds remains a DateTime bound;
subtracting two DateTime paths yields a Number and cannot bound DateTime.

An active `RangeMin` is an inclusive lower bound and an active `RangeMax` is an
inclusive upper bound. They intersect the native numeric type limits and any
static `NumberMin` / `NumberMax` constraints. Clients MUST apply that same
intersection to final validation and to numeric, DateOnly, DateTime, and TimeInterval
GUI controls. A changing dependency value MUST update the live GUI constraint.
DateOnly bounds use the `yyyyMMdd` wire representation at the schema boundary,
but Data Validation evaluates the value as a real Gregorian calendar date.
DateTime remains a distinct UI/wire and expression type but is constraint-
backed as an unsigned integral Unix timestamp. Static Number constraints use
that wire representation; dynamic Range expressions require a DateTime result.
Their intersection applies to both its local date/time chooser and final UTC
epoch-seconds validation.
`RangeMin` and `RangeMax` do not change the `NumberStep` origin or lattice. If
the active effective minimum exceeds the active effective maximum, the range
is unsatisfiable; a client MUST NOT swap the two bounds or otherwise invent a
valid interval.

A numeric field, including an editable request field, can use limits from the
current reference context. When its reference objects do not provide them, these expressions return
null and add no dynamic bound; ordinary numeric constraints still apply:

```typescript
/** Measured value
 * A numeric result constrained by the available server-provided limits.
 * @LibertasNumber(type=f64)
 * @LibertasRangeMin('{$.limits.minimum}')
 * @LibertasRangeMax('{$.limits.maximum}')
 */
value: number;
```

These curly references can directly constrain editable request fields. The
client reads the already received response; it does not fetch server state.
The server remains responsible for enforcing its current authoritative limits.

---

# 4. Path Access

## 4.1 Syntax

Current-data references use the square-bracket expression form of
[Libertas Schema Path](schema.md#current-data-references-in-validation-expressions).
Its canonical specification defines field/index syntax, owner roots, cursors,
array fan-out, result order, and null/collection conversion.

## 4.2 Path Resolution

Apply the [Libertas Schema Path resolution rules](schema.md#current-data-references-in-validation-expressions)
to the actual current data instance. The worked examples below illustrate those
rules in validation expressions; they do not define another path language.

## 4.3 Struct Access

Given:

```c
struct User {
    string name;
}
```

and concrete data:

```text
user.name = "John"
```

then:

```c
[user/name]
```

evaluates to:

```text
"John"
```

---

## 4.4 Null Values

Every field uses ordinary equality operators for its current editor value,
including a required field that has not received input yet.

```c
[email] == null
[email] != null
```

No special nullable-access syntax exists.

The comparison is valid even when the schema field is not `Nullable`. Client
editors use `null` as the initial state of many required fields, and rules are
allowed to describe that state before final structural validation.

For:

```c
[address] == null
```

the deeper path:

```c
[address/city]
```

also evaluates to:

```text
null
```

## 4.5 Reference paths

Curly expressions such as `{$.preferred.path,$.fallback.path}` use the
[shared path contract](schema.md#availability-null-and-interaction-ownership) for their syntax, lookup context, and
availability. During static rule checking their result is schema-unknown and
nullable. The first-composition guard is illustrated in §20.8.

---

# 5. Union Types

A schema MAY define a union:

```c
A | B | C
```

Every concrete union value has one active runtime type.

Example:

```c
struct A {
    int x;
}

struct B {
    string y;
}

A | B item;
```

---

## 5.1 `is` Operator

The `is` operator tests the actual runtime type of a value.

```c
[item] is A
[item] is B
```

Its result is boolean.

Formally:

\[
x\ is\ T =
\begin{cases}
true & \text{if }x\text{ has runtime type }T\\
false & \text{otherwise}
\end{cases}
\]

If the left operand is `null`:

```c
null is T
```

evaluates to:

```text
false
```

Negation uses the standard `!` operator:

```c
!([item] is A)
```

---

## 5.2 Path Access Through Unions

Libertas Schema Path follows the [active Union payload](schema.md#current-data-references-in-validation-expressions).
For `item = A { x: 10 }`, `[item/x]` returns 10 and `[item/y]` returns null when
y belongs only to another variant. This uses the same absence rules as missing
fields and indices; no special Union-access syntax is introduced.

---

# 6. Array Access

## 6.1 Explicit Index

The [Libertas Schema Path index form](schema.md#current-data-references-in-validation-expressions) selects array elements, for example:

```c
[items/0]
[items/0/name]
```

If the index does not exist, the path resolves to `null`.

---

## 6.2 Cursor traversal

`all`, `any`, `none`, and `collect` bind a cursor to each concrete element of
the arrays selected by their source path. A cursor is the element node itself,
not its array index:

```c
all(item in [items],
    [item/value] >= 0
)
```

`[item]` denotes the bound node and `[item/value]` starts path resolution from
that node. A cursor always retains the concrete absolute path used to reach its
node, so the example's second element is rendered diagnostically as
`[items/1/value]`.

A source path MAY cross other arrays before reaching its terminal arrays. When
a named segment remains and the current node is an array, the same segment is
resolved from every element. The terminal arrays are concatenated in path and
element order, then their elements are bound in that order:

```c
all(item in [groups/items],
    [item/value] >= 0
)
```

Here `[groups/items]` selects every `items` array under every element of
`groups`. No index variable is needed, and there is no ambiguous index owner.

The former index-binding form, such as
`all(i in [items], [items/i/value] >= 0)`, is invalid. There is no compatibility
interpretation for it.

---

# 7. Boolean Semantics

## 7.1 Boolean Context

Boolean values evaluate normally:

```text
true  -> true
false -> false
```

`null` is implicitly converted to boolean `false`:

\[
Bool(null)=false
\]

Therefore:

```c
[enabled]
```

is valid when `[enabled]` has type:

```text
bool
```

or:

```text
bool?
```

If `[enabled]` is `null`, it evaluates as `false`.

---

## 7.2 No General Truthiness

Non-null values other than booleans MUST NOT be implicitly converted to boolean.

The following are not valid boolean expressions merely because their values are non-null:

```c
[count]
[name]
[object]
[array]
```

Explicit comparison MUST be used:

```c
[count] != null
[name] != null
[object] != null
[array] != null
```

Thus the only boolean coercion defined by this specification is:

```text
bool  -> itself
null  -> false
```

---

## 7.3 First-non-null value fallback

Single `|` is a value-preserving, left-to-right null-fallback operator:

```c
expr1 | expr2 | expr3
```

The evaluator returns the first operand whose value is not `null`, without
changing that value's type. It evaluates no later operand after finding a
winner. `false`, numeric zero, an empty string, an empty array or collection,
and an object are all non-null and therefore win. The result is `null` only
when every operand evaluates to `null`.

```c
([preferredLimit] | [legacyLimit] | 100) >= 0
[preferredEnabled] | [legacyEnabled] | false
```

The operator may produce any supported value type inside an expression, but
the complete rule expression must still be Boolean. There is no additional
truthiness conversion: a numeric, string, array, collection, or object result
must be used with an explicit comparison or another type-appropriate operator.

The result's possible static types are the left operand's non-null types plus
the right operand's possible types only when the left operand may be null. A
chain is left-associative and applies that rule repeatedly.

---

# 8. Logical Operators

The language provides C-style logical operators:

```c
!
&&
||
```

Precedence, from highest to lowest:

```text
!
&&
||
```

`&&` and `||` MUST use short-circuit evaluation.

Example:

```c
[address] != null &&
[address/city] != null
```

If `[address]` is `null`, the second expression need not be evaluated.

Likewise:

```c
[item] is A &&
[item/x] > 0
```

---

# 9. Equality Operators

The language provides:

```c
==
!=
```

`null` is an ordinary value for equality comparison.

```c
[x] == null
[x] != null
```

The following identities apply:

```text
null == null  -> true
null != null  -> false
value == null -> false
value != null -> true
```

for every non-null `value`.

Non-null equality uses internal semantic types. Integer and floating-point
Numbers are compatible with each other. DateOnly equals only DateOnly and
DateTime equals only DateTime; neither equals a Number merely because its wire
value is numeric.

---

# 10. Relational Operators

The language provides:

```c
<
<=
>
>=
```

Examples:

```c
[age] >= 0
[min] <= [max]
[value] >= [min] && [value] <= [max]
```

Operands MUST be compatible according to the schema's type system.

Ordinary integers and floating-point Numbers are mutually comparable.
DateOnly is comparable only with DateOnly, and DateTime only with DateTime;
their storage encodings do not make them comparable with a Number. Strings are
comparable only with strings.

A relational comparison involving `null` evaluates to `false`.

For example:

```c
[item/x] > 0
```

evaluates to `false` if `[item/x]` resolves to `null`.

---

# 11. Arithmetic Operators

The core language provides:

```c
+
-
*
/
%
```

and unary:

```c
+
-
```

Example:

```c
[end] - [start] <= [maximum_duration]
```

Numeric literals and decoded scalar decimal text are evaluated as exact rational
values. Addition, subtraction, multiplication, division, remainder, equality,
and ordering preserve the low digits of i64 Money and do not use binary64
intermediates. Division does not round a repeating fraction; remainder uses a
quotient truncated toward zero. Each reduced numerator and denominator is
limited to 256 decimal digits. Arithmetic exceeding that limit produces null,
as does division by zero. This evaluation representation does not change the
schema's integer/floating semantic types or any wire encoding. Ordinary numeric
UI controls may still use floating-point coordinates; Money bounds and final
checks retain exact values throughout.

Ordinary integer and floating-point operands use numeric arithmetic. Integer
results remain internally typed as integers except that `/` produces a Number;
an operation involving a floating-point Number produces a Number.

DateOnly and DateTime have these additional, exhaustive arithmetic overloads:

```text
DateOnly + integer   -> DateOnly, shifted by Gregorian calendar days
integer + DateOnly   -> DateOnly, shifted by Gregorian calendar days
DateOnly - integer   -> DateOnly, shifted by Gregorian calendar days
DateOnly - DateOnly  -> Number, elapsed calendar days

DateTime + integer   -> DateTime, shifted by Unix seconds
integer + DateTime   -> DateTime, shifted by Unix seconds
DateTime - integer   -> DateTime, shifted by Unix seconds
DateTime - DateTime  -> Number, elapsed seconds
```

The integer operand MUST be internally typed as an integer; an integral value
whose declared or derived type is floating point is not sufficient. DateOnly
arithmetic MUST use Gregorian calendar rules: a DateOnly path carrying the wire
value `20240228` plus `1` evaluates to DateOnly `20240229`, and plus `2`
evaluates to DateOnly `20240301`. It MUST NOT add to the numeric `yyyyMMdd`
encoding. DateOnly/DateTime mixed
arithmetic, integer-minus-date, unary `+`/`-` on a date, and `*`, `/`, or `%`
on a date are invalid.

If an arithmetic operand evaluates to `null`, the arithmetic result is `null`.

Therefore:

```c
[a] + [b]
```

evaluates to `null` if either operand is `null`.

An undefined arithmetic operation, such as division by zero, also evaluates to `null`.

This keeps expression evaluation total for schema-compliant data.

---

# 12. Collection Operations

Traversal sources in this section MUST be data paths that resolve only to
arrays. Arrays selected through multiple parent-array elements are concatenated
in deterministic path and element order. A selected `null` or missing array
contributes no elements.

## 12.1 `all`

```c
all(node in [array_path], expression)
```

Returns `true` iff the expression evaluates to boolean `true` for every array element.

For an empty array:

```text
all(...) -> true
```

Example:

```c
all(item in [items],
    [item/price] >= 0
)
```

---

## 12.2 `any`

```c
any(node in [array_path], expression)
```

Returns `true` iff the expression evaluates to boolean `true` for at least one array element.

For an empty array:

```text
any(...) -> false
```

---

## 12.3 `none`

```c
none(node in [array_path], expression)
```

Returns `true` iff the expression evaluates to `false` for every array element.

For an empty array:

```text
none(...) -> true
```

---

## 12.4 `collect`

```c
collect(node in [array_path], predicate, projection_path)
```

`collect` returns an ordered collection and MUST process each bound cursor in
this order:

1. evaluate `predicate` against the concrete cursor;
2. if and only if it is boolean `true`, resolve `projection_path`; and
3. append the resolved projected values in path order.

The predicate is therefore a filter, not a condition evaluated after
projection. This is essential for Union values:

```c
collect(node in [nodes],
    [node] is Active,
    [node/device]
)
```

Only an `Active` concrete Union variant is projected. Other variants do not
need a `device` field, their projection is not evaluated, and they contribute
nothing. A projection that crosses arrays is flattened into the result. The
result preserves duplicates; it is a collection (bag), not a mathematical set.

An empty source or a predicate that never evaluates to `true` produces an
empty collection.

## 12.5 `is_unique_set`

```c
is_unique_set(collection)
```

Returns `true` exactly when no two collection elements compare equal under the
ValidationRules `==` semantics. Empty and single-element collections are
unique. Two `null` elements are duplicates.

`all` is not overloaded for this purpose. Its predicate is evaluated
independently for one bound cursor and returns only a Boolean; it has neither
collection state nor another-element binding with which to detect duplicates.

The canonical Union-filtered uniqueness rule is:

```c
is_unique_set(
    collect(node in [nodes],
        [node] is Active,
        [node/device]
    )
) : "ACTIVE_DEVICE_MUST_BE_UNIQUE";
```

## 12.6 `count`

```c
count([array])
```

returns the number of elements in an array or a `collect` result.

Example:

```c
count([items]) >= 1
```

If its argument resolves to `null`, `count()` evaluates to `null`.

---

## 12.7 Missing or Null Traversal Arrays

A missing or `null` terminal array contributes zero elements to a traversal.
Consequently, the usual empty-collection identities apply: `all` and `none`
return `true`, `any` returns `false`, and `collect` returns an empty collection.
Use an explicit path null test when array presence is itself required.

---

# 13. String Operations

The core specification defines:

```c
length(string)
```

which returns the number of characters in a string.

Example:

```c
[name] != null &&
length([name]) > 0
```

If the input is `null`:

```text
length(null) -> null
```

Additional string predicates MAY be defined by future revisions.

## 13.1 String resource references

```c
resource([status])
```

takes a FormattedText value and returns its string resource **key** as a
String, or null when the value is null. The key identifies the resource
template, not its localized text, so comparing it gives the same result in
every locale. Arguments do not change the key. Studio rejects `resource(...)`
on a value that is not FormattedText, and a string literal compared with a
`resource(...)` result that is not a key the package's `StringResources`
declares.

```c
resource([status]) == "STATUS_NOT_STARTED" || resource([status]) == "STATUS_NONE"
```

---

# 14. Expression Grammar

The normative grammar is:

The `current_data_schema_path` and `reference_object_schema_path` productions
are defined once in [Libertas Schema Path](schema.md#current-data-references-in-validation-expressions).

```ebnf
rule_set
    = [ "FinalOnly" "=" boolean_literal ";" ]
      rule_pair
      { rule_pair } ;

rule_pair
    = expression ":" string ";" ;

boolean_literal
    = "true" | "false" ;

expression
    = logical_or ;

logical_or
    = logical_and
      { "||" logical_and } ;

logical_and
    = equality
      { "&&" equality } ;

equality
    = relational
      { ("==" | "!=") relational } ;

relational
    = type_test
      { ("<" | "<=" | ">" | ">=") type_test } ;

type_test
    = fallback
      [ "is" type_name ] ;

fallback
    = additive
      { "|" additive } ;

additive
    = multiplicative
      { ("+" | "-") multiplicative } ;

multiplicative
    = unary
      { ("*" | "/" | "%") unary } ;

unary
    = ("!" | "+" | "-") unary
    | primary ;

primary
    = literal
    | path
    | reference_path
    | array_operation
    | function
    | "(" expression ")" ;

path
    = current_data_schema_path ;

reference_path
    = reference_object_schema_path ;

array_operation
    = "all"  "(" identifier "in" path "," expression ")"
    | "any"  "(" identifier "in" path "," expression ")"
    | "none" "(" identifier "in" path "," expression ")"
    | "collect" "(" identifier "in" path "," expression "," path ")"
    | "count" "(" expression ")"
    | "is_unique_set" "(" expression ")" ;

function
    = "length" "(" expression ")"
    | "resource" "(" expression ")" ;

literal
    = integer
    | floating_point
    | string
    | "true"
    | "false"
    | "null" ;

type_name
    = identifier ;
```

---

# 15. Operator Precedence

From highest precedence to lowest:

```text
1.  ()
2.  path access
3.  unary ! + -
4.  * / %
5.  + -
6.  |
7.  is
8.  < <= > >=
9.  == !=
10. &&
11. ||
```

Parentheses SHOULD be used when they improve clarity.

Example:

```c
([item] is A && [item/x] > 0) ||
([item] is B && [item/y] != null)
```

---

# 16. Static Rule Validation

Before accepting a schema, Libertas Studio MUST statically verify that:

1. operators other than `|` are applied to compatible types;
2. `|` preserves possible operand types and admits its right operand as a
   result only when its left operand may be null;
3. every complete rule and every predicate produces `bool` or possibly `null`;
4. arithmetic operators apply only to the ordinary numeric and date overloads
   in section 11, preserving the internal result type;
5. `is T` references a schema-defined type;
6. cursor variables are used only as the first segment of a data path and only
   within their lexical traversal scope;
7. every traversal source path resolves only to arrays, including when it
   selects several arrays by crossing parent arrays;
8. a path is potentially resolvable according to at least one schema-valid structural route;
9. every `collect` predicate is boolean and its projection is a data path
   potentially resolvable for the bound cursor; a direct `[cursor] is Variant`
   predicate narrows projection validation to that Union variant;
10. `is_unique_set` and `count` receive a collection or array;
11. every rule's string is a declared package `StringResources` key with a
   non-empty default-locale message;
12. `FinalOnly`, when present, is the single leading directive and its value is
   exactly `true` or `false`; and
13. `resource(...)` receives a FormattedText value, and a string literal
   compared with its result is a key that the package's `StringResources`
   declares (section 13.1).

Square paths are validated against the attribute evaluation root. Reference
paths are syntax-checked but their value type remains unknown and nullable,
because the referenced response may use a different compatible schema.

A path does not need to apply to every union alternative.

For example, given:

```c
A { int x; }
B { string y; }

A | B item;
```

both:

```c
[item/x]
[item/y]
```

are valid rule paths.

Their applicability is determined from the concrete runtime value.

A path that cannot exist under any value permitted by the schema SHOULD be rejected as a rule-definition error.

---

# 17. Evaluation Purity

Validation expressions MUST be pure.

A rule MUST NOT:

- mutate data;
- create data;
- delete data;
- perform I/O;
- access unprovided ambient state;
- read the current time from outside the supplied inputs;
- generate nondeterministic values;
- invoke arbitrary executable code.

Reading the supplied `referencedData` is permitted and does not access
unprovided ambient state. Evaluation does not fetch a response or consult
interaction history.

For fixed schema interpretation, `rule`, `data`, and `referencedData`:

\[
Eval(rule,data,referencedData)
\]

MUST always produce the same result.

---

# 18. Validation Result

The same expression language and evaluation semantics apply when a client
performs incremental validation during editing and when it performs final
validation. There is no separate relaxed or final expression dialect.
Incremental validation reports the rule result for the current, possibly
incomplete datum after omitting declarations whose leading directive is
`FinalOnly=true` or whose directive is omitted. Final validation evaluates every declaration, including
those final-only rule sets, and combines the result with all applicable static
schema checks.

For schema \(\Sigma\), concrete or partially entered `data`, supplied
`referencedData`, and rule set:

\[
R=\{rule_1,rule_2,\dots,rule_n\}
\]

define:

\[
Result_i(data,referencedData) = Bool(Eval(rule_i,data,referencedData))
\]

For that same `referencedData`, the data is valid iff:

\[
\boxed{
Valid(data,referencedData)=
\bigwedge_{i=1}^{n}Result_i(data,referencedData)
}
\]

Consequently:

```text
every rule true  -> VALID
any rule false   -> INVALID
```

Because `null` becomes `false` in boolean context:

```text
top-level null -> INVALID
```

For `INVALID`, a Libertas client MUST display the failing rule pair's localized
`StringResources` message. Evaluate applicable cumulative rule sets in their
supplied order and pairs within each set in declaration order; stop at the
first false rule. Skip final-only sets during incremental evaluation. Reporting
the failure MUST preserve normal short-circuit behavior and MUST NOT evaluate
skipped branches, later rules, or later quantifier elements to construct an
explanation.

The supplied message owns the explanation. Clients do not construct, rank, or
render structural failure certificates. If a trusted installed document is
unexpectedly missing the message or supplies only whitespace, report a generic
validation failure; do not expose expressions or entered values as a fallback.
A separate structural diagnostic may still reject a required field that remains
`null` at submission time.

Let `StaticValid` include mandatory/nullability, type, range, size, uniqueness,
and every other ordinary schema constraint. These checks receive the same
`referencedData` where applicable, including dynamic bounds. Final task data
is accepted iff:

\[
\boxed{
FinalValid_\Sigma(data,referencedData)=StaticValid_\Sigma(data,referencedData)\land Valid_\Sigma(data,referencedData)
}
\]

On final validation failure, keep the editor and its data. When the validator
identifies a failing field or a rule-owning group, show an error icon and the
explanation at that row, expand collapsed ancestors, and scroll the first
failure into view. A promoted header uses its owning row; a hidden field uses
its nearest visible owner. Do not replace the field’s normal control with an
inline editor: Boolean choices, enumerations, Unions and other pickers retain
their normal interactions. Function-wide rules without a field owner remain
message-only; do not infer field blame from an expression.

The marker is transient client state, not encoded data. After an accepted edit,
recheck the marked field and clear its marker when its validation passes or
when that field is removed. Do not transfer the marker to a replacement List
row or Union branch. A later Submit may reveal the next failure. These local
markers do not change backend Error responses, which retain the protocol’s
message presentation.

A client Task Editor MUST evaluate every `FinalOnly=false` rule set after every
proposed node change. One synchronous UI action MAY be treated as one atomic
change when it necessarily performs several model mutations, such as selecting
a Union variant and materializing its fields. Node changes include scalar and
byte replacement, nulling or materialization, Union selection, and List
insertion, removal, replacement, or reordering.

The client MUST snapshot the complete editable root set before the change,
because a field-local change can invalidate an ancestor, named-type, or
function rule. If incremental validation fails, the client MUST restore that
snapshot before presenting the violation as a client-native error toast. The
invalid value MUST NOT remain in task data. Declarations whose effective
`FinalOnly` value is `true` are not evaluated and cannot reject an incremental change. Incremental validation
does not run mandatory/nullability or other static final checks.

This rollback applies to the edits a user makes, on every client. A
programmatic assignment is not a proposed change and is never rolled back by an
incremental rule: a calculation result, a link return, a constrained-choice
reset, and a singleton selection stay in the editor, and final validation
governs them. A document import validates its complete candidate under
[its own rule](llm-document-transformation.md#client-workflow).

The client MUST evaluate all rule sets as part of final validation; a successful
incremental or final rule result cannot override `StaticValid=false`.

---

# 19. Constraint Completeness

The schema defines the structurally legal domain:

\[
D_\Sigma
\]

For fixed supplied `referencedData`, the rules define the semantically valid
subset:

\[
V_{\Sigma,R,referencedData}
=
\{
data\in D_\Sigma
\mid
Valid(data,referencedData)
\}
\]

Therefore:

\[
\boxed{
V_{\Sigma,R,referencedData}
=
\{
data\in D_\Sigma
\mid
\forall rule\in R,\ Bool(Eval(rule,data,referencedData))=true
\}
}
\]

If the Libertas schema and its validation rules are taken as the normative definition of the data model, then:

```text
Schema
+
Validation Rules
=
Semantic Type
```

or formally:

\[
\boxed{
SemanticType(\Sigma,R,referencedData)
=
\{data\in D_\Sigma\mid Valid(data,referencedData)\}
}
\]

Changing `referencedData` may change this subset without changing the schema,
rule, or current data. For example, the rule
`{$.maximum} == null || [] <= {$.maximum} : "ABOVE_MAXIMUM";` accepts `50`
when the supplied maximum is `100`, rejects it when the maximum is `40`, and
accepts it when there is no reference context because of its explicit null guard.

For a fixed schema, rule set, and `referencedData`, every schema-compliant
concrete data instance therefore has a deterministic validation result:

\[
\boxed{
Valid_{\Sigma,R,referencedData}: D_\Sigma
\rightarrow
\{VALID,INVALID\}
}
\]

---

# 20. Examples

## 20.1 Numeric Range

Schema:

```c
struct Range {
    float min;
    float max;
    float value;
}
```

Rules:

```c
[min] <= [max] : "INVALID_RANGE";

[value] >= [min] &&
[value] <= [max] : "VALUE_OUT_OF_RANGE";
```

---

## 20.2 Nullable Limit

```c
[limit] == null ||
[limit] >= 0 : "NEGATIVE_LIMIT";
```

---

## 20.3 Union

Schema:

```c
A {
    int x;
}

B {
    string name;
}

A | B item;
```

Rules:

```c
([item] is A && [item/x] >= 0) ||
([item] is B && [item/name] != null) : "INVALID_ITEM";
```

---

## 20.4 Union Projection Without Explicit Type Test

```c
[item/value] == null ||
[item/value] >= 0 : "NEGATIVE_VALUE";
```

This means:

> If `value` exists in the concrete union value, it must be non-negative.

---

## 20.5 Array

```c
count([items]) > 0 : "ITEM_REQUIRED";

all(item in [items],
    [item/name] != null &&
    [item/value] >= 0
) : "INVALID_ITEM";
```

---

## 20.6 Array of Union Values

```c
all(item in [items],
    ([item] is A &&
     [item/x] >= 0)
    ||
    ([item] is B &&
     [item/name] != null)
) : "INVALID_ITEM";
```

## 20.7 Named type, field, and function ownership

```typescript
/// @LibertasStringResources(ValidationMessages)
const ValidationMessages = {
    NONNEGATIVE: "Enter zero or more.",
    PERCENTAGE_MAXIMUM: "Enter a percentage no greater than 100.",
    INVALID_RANGE: "The lower limit cannot exceed the upper limit."
} as const;

/** @LibertasValidationRules('[] == null || [] >= 0 : "NONNEGATIVE";') */
export type NonNegative = number;

export interface Range {
    /** @LibertasValidationRules('[] == null || [] <= 100 : "PERCENTAGE_MAXIMUM";') */
    percentage: NonNegative;
}

/** @LibertasValidationRules('[minimum] == null || [maximum] == null || [minimum] <= [maximum] : "INVALID_RANGE";') */
export declare function configure(minimum: number, maximum: number, range: Range): void;
```

The `percentage` value must satisfy both the named-type and field-local rules.
The function rule sees minimum and maximum in one root. The explicit null guards
are valid even though the source types are not nullable, because the editor may not
have received user input yet. If one of those arguments remains `null` at final
submission, its validation rule may pass while the independent mandatory-field
check still rejects the task data.

## 20.8 Reference-context lookup

```c
[] == null ||
{$.limits.maximum,$.legacy.maximum} == null ||
[] <= {$.limits.maximum,$.legacy.maximum} : "ABOVE_PRIOR_MAXIMUM";
```

This field-level rule uses the first non-null maximum found in the current
reference context under the shared source-then-path ordering. During first-time
composition with no reference context, every curly reference
is `null`. The explicit null guard makes this rule pass in that state without
evaluating the comparison against an absent maximum. Once a reference object
supplies a maximum, the same rule enforces it. A typed `|` fallback can instead
supply an application-selected limit when the referenced value is absent.

---

# 21. Core Design Principles

The Libertas Data Validation Rules language is based on the following principles:

1. **Validation operates on concrete data and incomplete client editor state.**
2. **Schema compliance and semantic validity are separate.**
3. **Current data uses `[a/b/c]`; external lookup uses explicit `{...}` paths.**
4. **A path resolves to `null`, one concrete value, or an ordered collection when array fan-out selects multiple nodes.**
5. **Union types require no special path-access mechanism.**
6. **`is` tests the actual runtime union type.**
7. **Null values use ordinary `== null` and `!= null`, including on required fields during editing.**
8. **A rule that accepts temporary `null` never overrides final mandatory-field validation.**
9. **`FinalOnly` defaults to `true`; it is a rule-set directive, not a schema attribute, and only explicit `false` opts that declaration into incremental validation.**
10. **`null` implicitly becomes `false` in boolean context.**
11. **No other general truthiness conversion exists.**
12. **Single `|` returns the first non-null operand unchanged and short-circuits every later operand.**
13. **Rules are pure and deterministic for fixed schema interpretation, rule, data, and supplied referencedData.**
14. **Arrays use cursor traversal and collection operators rather than general-purpose loops.**
15. **For fixed referencedData, a valid rule set defines a total VALID/INVALID decision over every schema-compliant concrete data instance.**

The schema and pure rules define the model; each evaluation additionally
supplies `referencedData`:

\[
\boxed{
\text{Libertas Semantic Data Type}
=
\text{Schema}
+
\text{Pure Validation Rules}
}
\]
