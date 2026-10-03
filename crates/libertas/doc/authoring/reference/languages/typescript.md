# Libertas TypeScript Developer's Manual

This manual provides a quick reference for writing TypeScript code for the Libertas Platform. Libertas is fundamentally about **self-describing software**. By writing standard TypeScript, the Libertas CLI and IDE automatically extract your code's documentation, structure, and decorators to understand its capabilities. A robust user interface is just one of the many great features automatically generated from this self-describing architecture, alongside API layers and Agent integrations.

## Source code is the ground truth

Libertas relies exclusively on information extracted directly from your TypeScript definitions. You do not need to configure any external JSON schemas or metadata files. The parser follows TypeScript compiler symbols from the entry module. In an ordinary package, exported non-ignored functions are the roots and only the minimum transitive type closure from their arguments and reachable schema-reference decorators is published. In a `TypesOnly` package, every public package-owned named type exported by the entry module is automatically a root unless ignored.

The TypeScript parser is authoritative for TypeScript annotation syntax,
compiler-symbol resolution, source traversal, and extraction. Libertas Studio's
shared schema validator is authoritative for semantic rules decidable from the
emitted parser result and its materialized dependencies; another language
parser must not duplicate those rules.

The parser records external package-qualified references and emits an empty
intermediate `AppDependencies` list. Studio assigns required exact `Version`
pins during the [dependency-first deployment workflow](../deployment.md).
Application source must never predict or manually maintain these pins.

For TypeScript SDK behavior, the public comments in
[`libertas-ts/index.d.ts`](https://www.npmjs.com/package/@libertas-ai/types) are the sole ground
truth. This manual is a non-normative usage summary. If it, an example, a test,
or another document disagrees with those SDK comments, follow the SDK comments.

The 64-bit SDK (0.2.0) represents device, endpoint, LAN-device, user, and peer
identities as `bigint` aliases. Keep those values and collection keys as bigint;
converting through `number` loses precision above `2^53`. Use `0n` only where an
API defines a system context. Transaction IDs, timers, and data-store handles
remain numbers. Rebuild Apps against this SDK for the matching Hub runtime.

## Project configuration baseline

Preserve an existing project's SDK and compiler pins. For a new ESM App, this
is the supported strict baseline; the `types` entry makes the SDK's ambient
Libertas declarations available without invented imports:

```json
{
  "compilerOptions": {
    "module": "nodenext",
    "target": "esnext",
    "lib": ["esnext"],
    "types": ["@libertas-ai/types"],
    "strict": true,
    "verbatimModuleSyntax": true,
    "isolatedModules": true,
    "skipLibCheck": true
  },
  "include": ["src/**/*.ts"]
}
```

Run the project's build/typecheck or at least `tsc --noEmit` before schema
generation.

---

## Documents and comments

Libertas uses specific comment styles to extract localized documentation for your components, ensuring regular code comments don't accidentally leak into the user interface.

Every authored schema-visible package, function, named type, parameter, field,
Enumeration member, Union variant, and both halves of a List must follow the
[schema documentation invariants](../documentation.md):
provide a non-empty natural label and description, explain meaning in ordinary
domain language, and keep implementation vocabulary out of localized text.
For every public function, additionally follow the
[developer function-comment invariants](../function-comments.md):
state the purpose first, cover every user-relevant capability, use active voice,
and name who performs each action.

1. **Package Documentation (`///`)**:
   - The triple-slash `///` comments define the **Package-level** documentation.
   - They should only be placed at the very top of your entry file (e.g., `index.ts`).

2. **Component Documentation (`/** ... */`)**:
   - Standard JSDoc block comments (`/** ... */` or `/* ... */`) are used for documenting types, interfaces, classes, enums, and properties.
   - **Short Document (First Line)**: The very first line of the comment is extracted as the **Short Document**, which is used as the Short Name or Display Name in the UI.
   - **Long Document (Remaining Lines)**: Any remaining lines after the first line are extracted as the **Long Document**, which serves as the detailed Description field in the UI or LLM tool call description. Long descriptions use HTML-style flow: one logical CR/LF break becomes exactly one space, while two or more logical breaks become one LF paragraph break. CRLF counts as one break, so leave a blank JSDoc line only when a paragraph break is intended.
   - **Enums & Union Variants**: Documenting an underlying type or enum member will correctly attach that documentation to the variant within the discriminated union.

3. **Custom Document Properties (`[PropertyName]`)**:
   - After the short and long documents, place a consumer-defined marker on its own line. Text beginning on the next line is the property value and continues until the next marker or the end of that comment section. Unknown properties are preserved but have no platform behavior; use one only when its consumer contract defines it.
   - Multiple properties are supported. They are emitted as string keys directly beside `DisplayName` and `Description` in document JSON.
   - `DisplayName`, `Description`, `List`, `StringResources`, `Strings`, `Enum`, and `DocumentVersion` are reserved. `Strings` is prohibited and is not a document field. A structural key at that location, such as `Name`, `Type`, `Locale`, or `Types`, is also unavailable.
   - With an array `----` divider, properties above the divider belong to the array and properties below it belong to each element.
   - Display names and descriptions are always present in generated documents. The JSON structure permits an empty value only so generated, legacy, or incomplete nodes can be represented; a codebot-authored schema node must always provide both a non-empty natural label and description. Every document uses format version `0.0.2` and one top-level package `StringResources` map. Per-function/per-type `Strings` fields do not exist.
   - `[DefaultText]` is one-line localized default text on a `String` field or an `Endpoint` server field. Unlike the schema `Default` value, it may differ by locale.
   - `[DefaultTaskName]` is a localizable default task name and is valid only in a function's documentation. It does not replace the function's stable native `Name` or localized `DisplayName`.
   - Put every ordinary label/description line and extension block before the
     first `@Libertas...` tag. Document extraction stops at that tag.
   - The complete user-language, standard-extension, reserved-name, List-side,
     and translation contract is in the
     [schema documentation invariants](../documentation.md).

4. **Code Comments (`//`)**:
   - Standard double-slash `//` comments are **completely ignored** by the schema generator. Use them freely for internal code documentation.

```typescript
/// Garden irrigation
/// Helps people schedule and manage watering for their planted areas.

/**
 * Configure watering
 * The watering controller follows the user's schedule and lets the user adjust
 * each area's watering. Developed by Example Developer (example_developer).
 * [DefaultTaskName]
 * Back garden watering
 */
export function configure(
    /**
     * Watering choices
     * The user chooses the schedule and other settings for this watering plan.
     */
    configuration: Configuration
): void {}

/**
 * Watering settings
 * Describes the choices used to schedule watering for planted areas.
 */
export interface Configuration {
    /**
     * Schedule name
     * The name people use to recognize this watering schedule.
     * [DefaultText]
     * Weekday schedule
     */
    scheduleName: string;
}

/**
 * Controller settings
 * Describes how the watering controller is identified for this watering plan.
 * @LibertasExport
 */
export interface MyDeviceConfig {
    // This is an internal code comment and will NOT appear in the UI
    /**
     * Controller code
     * The user enters the code assigned to the watering controller.
     */
    id: string;
}
```

Replace the example Plaza identity before publication. TypeScript parameter
documentation may come from `@param` text or the parameter declaration; keep
them semantically identical when both exist. Union variant text can come from a
discriminator member, branch, or payload type and must never conflict across
those sources.

The exhaustive current source-tag catalog is the
[schema annotation inventory](../annotations.md).
Do not infer an authoring tag from a generated schema attribute or a
parser-internal alias.

Unless a block begins with package-level `///` text, its declarations are an
excerpt to place inside an entry module that already has the required package
label and description. Such declaration excerpts remain copyable when their
referenced types are declared and documented elsewhere. A deliberately
incomplete grammar or precedence illustration is explicitly labeled
**non-copyable**.

---

## Core Development Concepts

1. **Global Types**: All TypeScript projects must reference `@libertas-ai/types` as global types.
2. **Standard TypeScript**: Write standard TypeScript. Your code must compile successfully (`tsc --noEmit` should pass) before the schema can be generated.
3. **Entry Point**: The schema generator always starts scanning at `index.ts`.
4. **Exported Types**: The schema includes all exported functions and any types their arguments reference (directly or indirectly).

---

## The Libertas Type System

Your TypeScript coding style directly defines the schema of your software. Because Libertas software is self-describing, the types you choose dictate how the system understands your data, which in turn determines the resulting UI representation and API boundaries. You can use standard TypeScript features combined with Libertas native types to organize your data intuitively.

### Native Global Types

When defining properties and function parameters, use the native Libertas types
provided by the `@libertas-ai/types` package to ensure your schemas map correctly
to the platform's features:

- **`LibertasEndpoint`**: Represents a protocol endpoint for interactions with humans, AI, and other software.
- **`LibertasDateTime`**: A timestamp (Unix epoch/seconds). In the UI, this automatically displays in the user's local timezone and parsed back to UTC on save.
- **`LibertasTimeOnly`**: Provides a 24-hour time selector in the UI.
- **`Uint8Array`**: Represents binary data/bytes, mapping directly to the platform built-in type `"Bytes"`.
- **`LibertasUser`**: Presents a system user or user group picker. Use explicit [User and group defaults](../schema.md#user-and-group-defaults) for initial suggestions; the picker does not infer the current user.
- **`LibertasDevice`**: Presents a physical or logical device picker.
- **`LibertasVirtualDevice`**: Presents a software-emulated virtual device picker.
- **`LibertasLanDevice`**: Presents a WiFi or Ethernet network device picker.

For a MIME-aware `Uint8Array` field, write
`/** @LibertasMimeTypes("image/png,image/jpeg") */` above the declaration.
The parser emits `MimeTypes`; Studio validates its filter and effective Bytes
host. The [MIME-aware Bytes contract](../schema.md#mime-aware-bytes) owns LMB1
encoding, empty values, and client presentation. A TypeScript App emits the
LMB1 byte sequence as its ordinary `Uint8Array` value; an empty `Uint8Array`
is the valid media-undefined value. Keep unannotated `Uint8Array` fields for
raw binary protocols.

### Matter Status Values

The TypeScript SDK represents a Libertas Matter client status as an unsigned
32-bit number:

The following expression is ordinary runtime code, not a schema declaration,
so it creates no localized document node.

```typescript
const packedStatus = ((clusterStatus ?? 0) << 8) | interactionModelStatus;
```

Bits 0-7 contain the Matter interaction-model status, bits 8-15 contain the
optional cluster-specific status, and bits 16-31 contain a Libertas Hub status.
A zero cluster byte means that no cluster-specific status is present.
`LibertasMatterHubStatus.Superseded` is `0x00010000`. It rejects a queued
command replaced by a later command, even when their payloads are identical;
the retained caller is notified only after the surviving interaction completes.
Only the surviving command receives its own success or error result.
For writes, it reports a requested attribute value differing from the value
finally written by the pooled Matter interaction. See the
[Matter status contract](../matter/status.md#command-supersession).
This encoding is used by `LibertasIdStatus` and numeric `LibertasCommandRsp`
and `LibertasDeviceRsp` values received by a Matter client.

A Matter virtual device may return only the standard low 16 bits. Therefore a
plain `Matter.Status` value can be passed directly when no cluster status is
needed; a virtual device must never return `LibertasMatterHubStatus` values.
The TypeScript runtime rejects a virtual-device status with nonzero upper bits.

Decode a returned status with `packedStatus & 0xff` for the interaction-model
status and `(packedStatus >>> 8) & 0xff` for the cluster status.
Decode the Hub field with `packedStatus >>> 16`. See the
[Libertas Matter client status contract](../matter/status.md) for supersession,
completion timing, and error-override details.

### Arrays & Interfaces

#### Interface & Class Composition
You can build reusable models using standard TypeScript `extends` clauses for classes and interfaces. The platform will deeply explore all inheritance chains and automatically resolve all properties from the parent base classes into your component's definition. Properties, attributes, and comments from base interfaces are automatically inherited and displayed as part of the child object in the UI. The child can selectively override any inherited property definition or comment by re-declaring it.

#### Array Documentation Dividers
When documenting a `T[]` or `Array<T>`, you often need to provide instructions for the list as a whole, as well as instructions for a single item. Libertas supports **Array Divider Handling** using four hyphens (`----`):

```typescript
/// @LibertasStringResources(ValidationMessages)
const ValidationMessages = {
    NONNEGATIVE: "Enter zero or more.",
    PERCENTAGE_MAXIMUM: "Enter a percentage no greater than 100.",
    INVALID_RANGE: "The lower limit cannot exceed the upper limit."
} as const;

/**
 * Watering settings
 * Describes the planted areas included in one watering plan.
 */
export interface Config {
    /**
     * Watering zones
     * Lists the planted areas included in this watering plan, in watering order.
     * ----
     * Watering zone
     * Describes one planted area and the valve that supplies its water.
     * @LibertasSize(min=1)
     */
    zones: SprinklerZone[];
}
```
* **UI Presentation**: The UI uses the text above the divider to label the array container, and uses the text below the divider to label each individual item within the list.

### Enumerations

To represent a closed set of numeric constant choices, you have three options. All of them resolve identically to a simple dropdown or selection list in the user interface. **Note: All enumeration variant values must be numeric.**
1. **Standard TypeScript Enum**: Use a standard TypeScript `enum`.
2. **Const Object Enum**: Use a read-only `as const` object. Because TypeScript separates its value and type spaces, you must also export a corresponding type alias to use the object's values as a type annotation.
3. **Const Enum File**: Place a `@LibertasConstEnum` JSDoc tag at the very top of a file. Every `const` variable declaration within that file will automatically be parsed as an independent choice for that enumeration. This is particularly useful for aggregating sets of disparate constants into a single logical Enum type.

#### Code examples

The first two excerpts are alternative declarations in an already documented
package. The third starts a separate `error_codes.ts` file; it appears beside
the alternatives only for comparison.

```typescript
// 1. Standard TypeScript Enum
/**
 * Power state
 * Describes whether equipment is supplying power.
 */
export enum PowerState {
    /**
     * Powered on
     * The equipment is supplying power and can operate.
     */
    On = 1,

    /**
     * Powered off
     * The equipment is not supplying power.
     */
    Off = 0,
}

// 2. Const Object Enum
/**
 * Fan speed
 * Describes the airflow level selected for a fan.
 */
export const FanSpeed = {
    /**
     * Low airflow
     * The fan runs at its lowest available airflow level.
     */
    Low: 1,

    /**
     * Medium airflow
     * The fan runs at its middle available airflow level.
     */
    Medium: 2,

    /**
     * High airflow
     * The fan runs at its highest available airflow level.
     */
    High: 3,
} as const;
/**
 * Fan speed
 * Describes the airflow level selected for a fan.
 */
export type FanSpeed = typeof FanSpeed[keyof typeof FanSpeed];

// 3. Const Enum File (e.g., error_codes.ts)
/**
 * Connection problems
 * Describes why an attempted account connection could not be completed.
 * @LibertasConstEnum
 */
/**
 * Network unavailable
 * The application could not reach the remote service.
 */
export const ErrorNetwork = 100;

/**
 * Connection timed out
 * The remote service did not answer within the allowed time.
 */
export const ErrorTimeout = 101;

/**
 * Sign-in rejected
 * The remote service did not accept the supplied account authorization.
 */
export const ErrorAuth = 102;
```

Keep `error_codes.ts` dedicated to this Enumeration and declare exactly one
exported numeric member per statement so every member has its own JSDoc. The
App entry module imports that file as a namespace and exposes the schema type
through its own documented alias:

```typescript
import * as ConnectionProblemValues from "./error_codes";

/**
 * Connection problems
 * Describes why an attempted account connection could not be completed.
 */
export type ConnectionProblem = typeof ConnectionProblemValues;
```

### Discriminated Unions (Union Types)

Unlike Rust where union types are declared as enums with payloads, in TypeScript you use Discriminated Unions. **Note: The discriminator field used to distinguish between variants MUST be typed as a Libertas enumeration (such as a standard TS enum or const object enum).**

When modeling discriminated unions for protocol or data schemas, you can define variants as either separate named interfaces or anonymous inline type literals. The UI handles anonymous inline literals `{ type, payload }` seamlessly, providing the exact same input form as if you had used a named interface, recursively flattening fields directly under the variant.

The parser intentionally suppresses document text for the TypeScript
discriminator field itself. This compiler-oriented field is the sole ordinary
exception to the all-node documentation invariant. Document the observable
variant through its enum member, Union branch, or payload type instead.

### Optional Types

Optional modifiers (`?`) and nullable unions (`| undefined`) declare meaningful
absence. For ordinary optional user text, prefer a `string` with an empty
Schema Default and size bounds permitting zero, under the shared
[type-selection guidance](../app-contract.md#user-facing-and-non-user-facing-types):

```typescript
/**
 * Student details
 * Describes the personal details recorded for one student.
 */
export interface Student {
    /**
     * Middle name
     * Enter the student's middle name. Leave it empty if absent.
     * @LibertasDefault("")
     * @LibertasSize(min=0, max=120)
     */
    middleName: string;
}
```

This middle name starts empty and may stay empty. When absence must remain
distinct from a present value, use `amount?: number` or
`amount: number | undefined`, for example to distinguish an unknown amount from
known zero. Both forms emit the platform schema's `Nullable` attribute; a
Nullable value starts as valid absence. A size minimum constrains a supplied
String, not null. A non-Nullable String without a Fixed or applicable default
starts unanswered, even when its size minimum is zero.

* **Special Case**: If you define a property referencing a Union Type Alias with `| undefined`, the parser correctly resolves it as a reference to the type alias with the `Nullable` attribute, rather than flattening the union inline.

---

## Supported JSDoc Decorators

Libertas utilizes custom JSDoc decorators (starting with `@Libertas`) to control how the UI and interactions behave.
Codebots must generate the exact positional or named-property spelling shown in
the exhaustive [schema annotation inventory](../annotations.md).
Some older named-argument variations remain parser compatibility syntax, but
they are not additional public authoring forms. `@LibertasFormattedText` and
`@LibertasSingleton` are no-argument markers.

### General Decorators
- **`@LibertasExport`**: Parser/compiler helper that does not emit a schema attribute. It does not create a standalone root in an ordinary package and is not needed to root public types in a `TypesOnly` package.
- **`@LibertasIgnored`**: No-argument parser directive valid on an exported function or exported named type. It omits that declaration as a root. If an included root reaches an ignored type, parsing fails.
- **`@LibertasSingleton`**: No-argument function marker. It emits `Singleton=true` and allows at most one task for that App function on a Hub.
- **`@LibertasDefaultLocale(locale_name)`**: Package-level decorator (must be in `///` comments) setting the default locale for your package. Defaults to `"en"`. The locale is 1–128 ASCII alphanumeric characters in segments separated by one `-` or `_`; leading, trailing, and repeated separators are invalid.
- **`@LibertasStringResources(CONST_NAME)`**: Package-level decorator in the leading `///` comments. It resolves exactly one non-empty `const` object-literal catalog, emits its sorted keys on the schema package, and writes its LMF1 templates to the document's top-level `StringResources` object. It is invalid on functions, types, and fields.
- **`@LibertasTypesOnly`**: No-argument package-level marker in the leading `///` comments. It automatically publishes every public package-owned named type exported by the entry module unless ignored, without compiling or deploying executable code. Exported functions must carry `@LibertasIgnored`; other exported functions are rejected.
- **`@LibertasDefault(value)`**: Sets a default value for an input field. Enumeration member names or numeric defaults follow the [shared enumeration contract](../schema.md#53-enumerations). User and User-array defaults use the [identity-selector contract](../schema.md#user-and-group-defaults), for example `@LibertasDefault("current-user")`.
- **`@LibertasFixed(value)`**: Emits one fixed encoded value. On an Endpoint
  client the value instead identifies the peer task process as
  `developer-package::function`; the Endpoint selector remains editable, the
  compatible server set is constrained to that process, and its package is a
  mandatory dependency. Enum member Fixed values and Union member Discriminator values come from
  their native declarations; these attributes have independent meanings.
  On `LibertasUser`, Fixed is a [Hub-enforced security constraint](../schema.md#fixed-current-user-security-constraint); only `current-user` is valid, at a request root or within nested request values outside Lists, optionally Hidden.
- **`@LibertasSuppressLabel`** suppresses a scalar, struct, or Union row’s visual label while retaining its value and controls. On an array of structs or its type alias it suppresses only List-contributed member names and positions, retaining the List heading. Studio compiles it into the [Display Template](../presentation.md#display-template) and enforces [placement and page-title rules](../presentation.md#suppresslabel-label-free-value-rows). The former `@LibertasSuppressTitle` spelling is rejected. Promotion, flattening, row labels, and row titles have no decorator: write them as [`promote`](../presentation.md#tableheader-promoted-fields-and-union-selectors) (only on a field of a struct), [`flatten`](../presentation.md#flatten) (on a struct, array, or union), [`label`, and `title`](../presentation.md#labels-and-titles) rules in the project's `display.json`, beside `package.json` ([data-titled rows](../presentation.md#data-titled-rows)). The retired `@LibertasUIHeader` and `@LibertasUIName` are rejected.
- **`@LibertasUIIncludeDescription`**: No-argument client presentation marker for a schema data field or type. The UI node displays its localized long description inline together with its short display name. It does not change the typed value or wire encoding and is compiled into the [Display Template](../presentation.md#display-template) as its `include_description` hint.
- **`@LibertasPermissions(CONST_NAME)`**: Resolves a const string array into the comma-delimited `Permissions` schema attribute. Valid on functions, where every permission needs a matching non-empty string-resource reason, and on built-in Endpoint servers, where no reason resource is required.
- **`@LibertasAccessHost("host1,host2")`**: Function-only network declaration. Concrete DNS names and reserved `{network_access_local}` / `{network_access_external}` grants each need a non-empty same-key reason in package `StringResources`. See [App network access](../network-access.md).

- **`@LibertasReadOnly`**: Marks the field as read-only in the UI.
- **`@LibertasHidden`**: Hides the field from the UI entirely.
- **`@LibertasLLMToolCall(false)` / `@LibertasLLMToolCall(true)`**: On a protocol Union or Request variant, controls whether the Hub generates an LLM tool definition. A Union's value applies to its variants unless a variant overrides it; omission defaults to true. See [the shared attribute contract](../annotations.md#endpoint-and-protocol-annotations).
- **`@LibertasLLMDocTrans(instruction="...", validation="@.facts", doc_type="image/*", accept_multiple_inputs=true)`**: On a named Table interface, declares one AI Import action per editable Request occurrence. Only `instruction` is required; the other named arguments are optional. Its direct result fields use `@LibertasLLMDocField` or `@LibertasLLMDocField(allowEmpty=false)`. A non-Nullable Number defaults to `allowEmpty=true` and needs a meaningful Schema `Default`; a clearly blank box maps to that typed Default. See [document transformation](../llm-document-transformation.md#app-schema-authoring) for Schema Path facts, numeric conversion, source files, and application.
- **`@LibertasExcludeUI`**: Makes an Endpoint server carrying a resolved
  protocol schema, or a reachable `Request`/`SubscriptionRequest` variant,
  nonexistent to Control UI. The Hub Control service omits the marked server
  Endpoint from its UI-device list and LLM tool collection; generated clients
  and Hub LLM tool-definition generation omit the marked operation. Admin
  creation/editing still uses the complete schema. It is a no-argument marker
  and does not change protocol roles, wire representation, routing,
  subscriptions, or authorization. Every server Endpoint sharing one resolved
  `ProtocolSchema` root must use the same value; see the
  [Endpoint declaration contract](../protocols.md#declaring-an-endpoint).
- **`@LibertasPhysicalUnit("millimeter")`**: Declares a canonical CLDR core
  unit identifier on a numeric field. It takes exactly one quoted positional
  identifier and emits `PhysicalUnit=millimeter`; clients preserve that unit
  and localize its short visual and long accessibility names when supported by
  the platform ICU implementation. It cannot be
  combined with `@LibertasDateOnly`, `@LibertasTimeInterval`, or
  `@LibertasEnumSource`. Generated CLDR compound units such as
  `millimeter-per-day` are valid regardless of whether a particular client can
  currently display them; an unsupported client may omit unit decoration while
  preserving the value. The normative syntax and chart behavior are defined in
  [Client-side charting §5.5.1](../charts/2d.md#551-physical-units) and
  [Unicode UTS #35 unit identifiers](https://unicode.org/reports/tr35/tr35-general.html#Unit_Identifiers).
- `@LibertasMoney("USD", 2)`: Declares `@LibertasNumber(type=i64)` Number money in exact integer minor units and
  emits `Money=USD/2`. Currency and scale are fixed schema metadata; clients
  display currency and edit major units in place. See the [Money contract](../presentation.md#money)
  for supported scales, patterns, validation, and chart behavior.
- **`@LibertasFormat("pattern")`**: Display-only `Format`, compiled into the [Display Template](../presentation.md#display-template), on a
  `number`, `LibertasDateTime`, `LibertasTimeOnly`, or `@LibertasDateOnly`
  integer. Ordinary numbers use
  [Unicode UTS #35 number patterns](https://unicode.org/reports/tr35/tr35-numbers.html#Number_Format_Patterns);
  temporal hosts use
  [Unicode UTS #35 date/time patterns](https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns).
  The viewer's locale supplies symbols and names. The pattern never changes the
  raw value, wire encoding, validation, sorting, or chart geometry.
- **`@LibertasForeignType("developer-package::Type")`**: Declares the direct official type for this published compatible interface, class, type alias, or enum. It is invalid on fields and references. The value uses the exact `developer-package::Type` FQN syntax.

Use the normal imported or qualified TypeScript type name so the parser can
resolve its native declaration. Every named type published by an App, including a
compatible type, must be exported or re-exported from the App's root `index.ts`
under its exact declaration name. Renamed exports and duplicate flattened names
are rejected.

A resolved type outside the current package is automatically foreign and uses its
native owning App FQN. The owning package must be a Libertas App that exports the
exact type from its root. A compatible type remains published under its own App
FQN; `@LibertasForeignType` links it to the official type without changing its
identity or propagating to children. `ExternalNativeType` has been removed.
Ordinary reference FQNs are always inferred from TypeScript-native references. A
qualified reference may be quoted in a protocol/data-schema annotation, but it
must resolve and is converted to the owning App FQN. `@LibertasForeignType` is
the only FQN taken literally as schema identity. Every unqualified type name,
whether used in a type slot, heritage clause, or annotation, resolves only in
its declaration's actual TypeScript scope. A valid imported external name
becomes `developer-package::TypeName`; an unbound name is a parser error even if
another program file happens to declare the same terminal name.

```typescript
/// Shared definitions
/// Provides reusable equipment status details for compatible products.
/// @LibertasTypesOnly

/**
 * Equipment status
 * Describes an equipment condition and its attached measurement file.
 */
export interface SharedStatus {
    /**
     * Operator status note
     * Gives the free-form condition note entered by an equipment operator.
     */
    message: string;

    /**
     * Measurement attachment
     * Carries the measurement file supplied with this equipment condition.
     */
    payload: Uint8Array;
}
```

### Cross-language interoperable packages

Build every package under your own developer name; never use another developer's name as your package namespace. As a best practice, publish public data schemas in a `TypesOnly` package and make its source available.

If consumers need an equivalent schema in another language, create a compatible
package under a new package name. Each compatible public type declares exactly
one `@LibertasForeignType` containing its direct official type FQN. Other public
types may remain ordinary, and mappings may refer to different official Apps.

At runtime Libertas verifies wire structure, merges official attributes into a
transient graph, and applies explicitly declared compatible attributes as
overrides. The published compatible schema is unchanged. Localized type documents
are shared automatically while TypeScript-only discriminator nodes remain local.

```typescript
/// Portable shared definitions
/// Provides equivalent equipment status details for compatible products.
/// @LibertasTypesOnly

/**
 * Equipment status
 * Describes an equipment condition consistently across compatible products.
 * @LibertasForeignType("origin-shared_types::SharedStatus")
 */
export interface PortableStatus {
    /**
     * Operator status note
     * Gives the free-form condition note entered by an equipment operator.
     */
    message: string;
}
```

### Numeric Modifiers
- **`@LibertasNumber(type=u8|i8|u16|i16|u32|i32|u64|i64|u128|i128|f32|f64, min=..., max=..., step=...)`**: One numeric annotation for integer and floating fields. `type=` is mandatory whenever this annotation is used. DateTime requires `u64`; unannotated native `number` retains its `f64` mapping. Bounds and step are validated by Studio against the selected numeric type. Integer step defaults to 1. The former Integer/Float source spellings are removed.
- **`@LibertasRangeMin("expression")` / `@LibertasRangeMax("expression")`**: Each carries one Data Validation bound expression compatible with the host type on a Number or `LibertasDateTime` field/type. See [dynamic numeric ranges](../validation-rules.md#31-dynamic-numeric-ranges) for result typing, compatible dates, and GUI/final enforcement. These expressions have no message key, semicolon pair, or `FinalOnly` directive.
- **`@LibertasTimeInterval`**: In the Task Editor, this triggers a specialized `HH:MM` time interval input for users.
- **`@LibertasDateOnly`**: No-argument marker for an integer `number` whose value is a Gregorian calendar date in `yyyyMMdd` form. It emits `DateOnly=true`, keeps the integer wire representation, and generates a native date chooser in task configuration and runtime interaction. It cannot be combined with `@LibertasTimeInterval` or `@LibertasEnumSource`; defaults and fixed values must be valid eight-digit dates.
- **`@LibertasBitflags` / `@LibertasBitflags(TypeName)`**: The no-argument form projects a TypeScript enum into a localized Enumeration catalog. The one-type form projects that catalog onto an explicitly sized integer field without changing its integer wire value.
- **`@LibertasFormat("pattern")`**: Uses a portable Unicode LDML display
  pattern. Examples are `#,##0.00` for an ordinary number, `yyyy-MM-dd` for
  DateOnly, `HH:mm:ss` for `LibertasTimeOnly`, and
  `d MMMM yyyy, HH:mm z` for `LibertasDateTime`. It takes exactly one
  double-quoted positional string. DateOnly accepts date fields only, TimeOnly
  accepts time fields only, and DateTime uses the viewer's local timezone.

```typescript
/**
 * Service schedule
 * Describes the calendar day chosen for one scheduled service visit.
 */
export interface Schedule {
    /**
     * Service day
     * The user chooses the calendar day when the service should occur.
     * @LibertasNumber(type=u32)
     * @LibertasDateOnly
     * @LibertasFormat("yyyy-MM-dd")
     * @LibertasDefault(20260815)
     */
    serviceDay: number;
}
```

### Bitflag catalogs and integer fields

Use a TypeScript `enum` as the language-equivalent bitflag declaration. Put the
no-argument marker on the enum and name it from the integer field marker. The
enum may remain private because Studio fabricates schema metadata from it; the
enum itself is not the runtime or wire value.

```typescript
/**
 * Access choices
 * Describes the independent actions a person may be allowed to perform.
 * @LibertasBitflags
 */
enum Access {
    /**
     * No access
     * The person cannot view, change, or run the protected content.
     */
    None = 0,

    /**
     * View content
     * The person may view the protected content without changing it.
     */
    Read = 1 << 0,

    /**
     * Change content
     * The person may change the protected content.
     */
    Write = 1 << 1,

    /**
     * Run actions
     * The person may run the protected actions.
     */
    Execute = 1 << 2,
}

/**
 * Access settings
 * Describes the actions allowed for a person or group.
 * @LibertasExport
 */
export interface Settings {
    /**
     * Allowed actions
     * The user chooses each action that the person or group may perform.
     * @LibertasNumber(type=u8)
     * @LibertasBitflags(Access)
     */
    access: number;
}
```

The field form may instead use a qualified TypeScript-native path to a catalog
owned by another Libertas App:

```typescript
import * as sharedFlags from "shared-flags";

/**
 * Access settings
 * Describes the actions allowed for a person or group.
 * @LibertasExport
 */
export interface Settings {
    /**
     * Allowed actions
     * The user chooses from the action catalog published by the shared package.
     * @LibertasNumber(type=u8)
     * @LibertasBitflags(sharedFlags.Access)
     */
    access: number;
}
```

The parser resolves the declaration and emits
`Bitflags=shared-flags::Access`; it never copies the import alias into schema
identity. Because the enum is fabricated metadata, it may remain private, but
it must be declared under the exact name `Access` at the owning App's root
`index.ts`. During deployment planning Studio fetches/explores that package
from the selected Hub if it is not local, and its authoritative Hub version is
included in finalized `AppDependencies`.

The enum becomes an `Enumeration` with `Bitflags=true`; its members become
numeric children with compiler-evaluated `Fixed` values. Each nonzero value
must be a unique single bit. One zero value is allowed for a localized empty
label such as “None” or “N/A”. The field must be an integer wide enough for all
bits. Defaults, fixed values, task values, and runtime values may combine
declared bits but may not contain unknown bits.

Studio displays localized selected names in comma-delimited form and edits the
value with a multiple-choice chooser. The Hub is deliberately unaffected: it
sees, stores, and transports only the ordinary integer. `Bitflags` cannot share
a field with DateOnly, TimeInterval, EnumSource, Format, or PhysicalUnit.

### Strings and Arrays
- **`@LibertasEnum(A, B, C)`**: Parser-supported compatibility syntax that
  fabricates positional string choices. Those fabricated members have no
  source location where a codebot can provide each member's required natural
  label and description. Codebots must therefore prefer a documented native
  numeric enum and must not generate this positional form until Libertas
  defines a member-document syntax for it. Existing hand-authored uses remain
  parseable.
- **`@LibertasSize(min?, max?)`**: Restricts size for lists, strings, and bytes.
- **`@LibertasUnordered`**: Marks an array as an unordered set, affecting how arrays are merged or displayed.
- **`@LibertasUnique(level?: number)`**: Enforces uniqueness on a field, preventing users from entering duplicate values in a list. The optional `level` defines how many ancestor `List` arrays up the tree to check to define the uniqueness boundary.

### Union Types & Discriminators

The parser emits numeric `Discriminator` on each Union member, sorted by that
value. The values must be consecutive starting at zero; Studio rejects gaps
and other invalid mappings. It also emits
`UnionTagField=true` on its native tag field. `UnionTagField` is exclusively
ScriptLanguage metadata (such as TypeScript/Python), never Rust metadata. `@LibertasDiscriminator("kind")`
continues to select the native property and emits `UnionTagName`; it does not
set a numeric payload constraint. `Fixed` retains its ordinary value meaning.
See [the Union schema contract](../schema.md#54-unions).


- **`@LibertasDiscriminator("tag_name")`**: Explicitly marks the field used as the union discriminator. If omitted, the platform will attempt to automatically detect the discriminator field.

---

## Union Variant Documentation

When you represent complex states, commands, or data variants using TypeScript discriminated unions, Libertas aggregates your documentation to provide a clean user experience using a strict 3-tier precedence hierarchy.

### 1. Document Precedence

| Priority | Source Layer | Description |
| :--- | :--- | :--- |
| **Priority 1 (Highest)** | **Corresponding Discriminator Value** | JSDoc comments preceding the resolved `enum` member or `const` property declaration corresponding to the variant's discriminator value. |
| **Priority 2 (Middle)** | **Union Variant Node** | JSDoc comments preceding the variant type reference directly within the union type definition. |
| **Priority 3 (Lowest)** | **Variant Class/Interface** | JSDoc comments preceding the base interface, class, or type declaration of the variant itself. |

#### Non-copyable precedence schematic

The following fragment deliberately references omitted declarations and uses
precedence-layer labels as placeholder document text. It exists only to show
which of three comments wins. Do not copy it into an App; real declarations
must use one consistent, natural label and description for every shown node.

```typescript
export enum MessageType {
    /**
     * Priority 1: Discriminator Zone Info (Highest)
     * This JSDoc takes highest precedence and will be the final document.
     */
    ZoneInfoVal = 2,
}

/**
 * Priority 3: Default Zone Info (Lowest)
 * This is the fallback if no other comments exist.
 */
export interface ZoneInfoMessage {
    type: MessageType.ZoneInfoVal;
}

export type ZoneDataProtocol =
    | GetZoneInfoMessage
    | /**
       * Priority 2: Custom Zone Info (Middle)
       * Takes precedence over the class docs but is overridden by the discriminator docs.
       * @LibertasNextRequest("UpdateHoldOffMessage")
       */
      ZoneInfoMessage
    | UpdateHoldOffMessage;
```
*In this example, the UI will display **"Priority 1: Discriminator Zone Info"** to the user.*

### 2. Attribute and Decorator Merging
Unlike the Short Name and Description (which follow strict overriding precedence), **custom JSDoc decorators and attributes are merged cumulatively** across all three layers. All resolved attributes are combined, eliminating duplicates, to form the final variant metadata schema.

### 3. Discriminator & Matching Heuristics
The schema generator automatically locates union discriminators and matches union members:
* **Discriminator Types**: Supports `enum` values as well as property lookups in objects defined as `const` (using `as const`).
* **Discriminator Validation Rules**:
  * **Numeric Discriminators Only**: Union variant discriminator fields must resolve to a numeric type. Using string literals as union variant discriminators is strictly prohibited.
  * **First Property Requirement**: The discriminator field/property must be the very first property declared within each variant's body/definition.

---

## Data-Backed Enum Indexes

Use `@LibertasEnumSource("path")` when an unsigned integer stores the zero-based index of an item in another list in the same schema-data tree. The attribute emits `EnumSource`; its one positional string must be a [Libertas Schema Path](../schema.md#context-and-syntax) in the EnumSource context. Studio rejects signed integers, floats, non-number hosts, unresolved paths, and paths whose target is not a list.

```typescript
/**
 * Choice settings
 * Describes the available choices and the choice currently selected.
 */
export interface Settings {
    /**
     * Available choices
     * Lists the literal choices in the order people see them.
     * ----
     * Choice
     * Gives the literal name of one available choice.
     */
    choices: string[];

    /**
     * Selected choice
     * Identifies the choice currently selected from the available list.
     * @LibertasNumber(type=u16)
     * @LibertasEnumSource("$.choices")
     */
    selectedChoice: number;
}
```

Here, `selectedChoice = 0` refers to the first item in `choices`.

---

## Formatted Runtime Text

Use the no-argument `@LibertasFormattedText` marker on an effective `Uint8Array`
(`Bytes` in the schema), together with static `ReadOnly` under the
[placement and request-value contract](../presentation.md#static-read-only-placement-and-request-values).
The marker emits
`"FormattedText": "true"` without changing serialization.
The byte blob is the canonical resource key plus ordered, typed
`LibertasMessageArgument` values, including an empty argument list. Localized
templates use zero-based LMF1, and runtime payloads never contain resource
templates.

Do not use a `string` host containing Base64. Rendering would only decode that
String back to the same byte tuple, adding a pointless encode/decode cycle and
providing no benefit. Keeping formatted values as Bytes creates a sharp type
distinction from literal text, making developer mistakes less likely. App code
keeps the value as `Uint8Array`; if a generic JSON transport represents Bytes
as Base64, that is only the transport codec and never the field's semantic type.

The public SDK function `libertasFormattedText(resourceName, arguments)`
returns the canonical `Uint8Array`. App code ordinarily does not decode it;
clients resolve and render the LMF1 template at the presentation boundary.

```typescript
/// Equipment status
/// Lets connected products request and present localized equipment conditions.
/// @LibertasStringResources(StatusStrings)

const StatusStrings = {
    READY: "The equipment is ready.",
    DEVICE_STATUS: "{0,LibertasDevice} reports a temperature of {1,number,0.0}."
} as const;

/**
 * Check current status
 * Asks the equipment controller to report its current condition.
 * @LibertasRequest
 */
export interface ReadStatus { kind: 0; }

/**
 * Equipment condition
 * Describes the localized summary and detail reported for equipment.
 */
export interface StatusBody {
    /**
     * Status summary
     * Gives people a concise localized description of the current condition.
     * @LibertasReadOnly
     * @LibertasFormattedText
     */
    summary: Uint8Array; // canonical READY tuple with no arguments

    /**
     * Status detail
     * Identifies the equipment and describes its current condition.
     * @LibertasReadOnly
     * @LibertasFormattedText
     */
    detail: Uint8Array; // canonical DEVICE_STATUS tuple
}

/**
 * Current equipment status
 * Reports the equipment's current condition in localized text.
 * @LibertasResponse
 */
export interface Status {
    kind: 1;

    /**
     * Reported condition
     * Gives the localized summary and detail for the equipment.
     */
    body: StatusBody;
}

/**
 * Equipment status check
 * Describes how people check equipment and receive its current condition.
 */
export type StatusProtocol = ReadStatus | Status;

const summary = libertasFormattedText("READY", []);

function equipmentTemperatureDetail(
    equipment: LibertasDevice,
    temperatureCelsius: number,
): Uint8Array {
    return libertasFormattedText("DEVICE_STATUS", [
        { type: 1, value: equipment },
        { type: 10, unit_type: "temperature-celsius", value: temperatureCelsius },
    ]);
}
```

The App package owns the catalog. `StatusBody` carries a reusable, statically
read-only Bytes shape; a `TypesOnly` package may publish it without choosing a
direction. A concrete App that later emits it uses that App's package catalog.
Studio checks each use against the linked placement contract.

Locally, Studio validates the exact marker and effective Bytes host plus the
syntax and default-document membership of the optional package catalog. It does
not require every formatted field or output path to prove runtime key
membership. See
[Libertas Message Format](../message-format.md) for the LMF1 grammar,
Libertas Message Argument contract, encoded-type priority, and chart behavior.

---

## Protocol chart presentation

Chart metadata is part of the ordinary endpoint protocol schema. It adds no
payload bytes or chart-specific message. Apply `@LibertasChart(mark)` to an
effective `Response` or `SubscriptionData` payload with shape
`List<Table Row>`. Direct scalar row fields use
`@LibertasChartChannel(...)` and may use `@LibertasChartScale(...)` when they
own one scale-bearing channel. `@LibertasChartGuide(...)` configures the generic
guide subsystem independently of scale mapping. A scale's `kind` and a guide's
`position` are chart appearance, compiled into the [Display Template](../presentation.md#display-template).

```typescript
/**
 * Soil-moisture reading
 * Records the soil moisture observed at one time.
 */
export interface Sample {
    /**
     * Observation time
     * Identifies when the soil-moisture reading was observed.
     * @LibertasChartChannel(x, tooltip)
     * @LibertasChartScale(id=time, kind=utc)
     */
    at: LibertasDateTime;

    /**
     * Observed soil moisture
     * Reports the soil-moisture percentage measured at the observation time.
     * @LibertasChartChannel(y, tooltip)
     * @LibertasChartScale(id=value, kind=linear, zero=false)
     * @LibertasPhysicalUnit("percent")
     */
    value: number;
}

/**
 * Soil-moisture history
 * Shows how the observed soil moisture changes over time.
 * ----
 * Soil-moisture reading
 * Records the soil moisture observed at one time.
 * @LibertasSubscriptionData
 * @LibertasChart(line)
 */
export type History = Sample[];
```

`Chart` accepts primitive `point`, `line`, `area`, `bar`, `rect`, `rule`,
`arc`, `text`, and `polygon`, plus composition `layer`, `hconcat`, and
`vconcat`. Point, line, rule, and text have Cartesian and polar coordinate
forms; polygon accepts Cartesian or polar vertices and uses implicitly closed
`evenOdd` compound fills.
Theta and radius are quantitative numeric positions; their explicit scale kind
may be `linear`, `log`, `symlog`, `sqrt`, or `identity` with already normalized
`0...1` values, not a discrete or temporal kind.
For a primitive mark, the annotated value may be either `List<Table Row>` or a
carrier table with exactly one non-hidden direct `List<Table Row>` field. Every
other direct carrier field must use `@LibertasHidden`; those values remain in
the decoded response for typed `NextRequest`/`CopyFrom` transactions but never
become chart rows, channels, tooltips, accessibility content, or resource
usage. Compositions are tables whose non-hidden direct children independently
carry `Chart`; direct non-chart children are allowed only with
`@LibertasHidden`. A response/subscription Union variant may similarly wrap
exactly one chart-bearing field plus hidden transaction fields. The client
retains the complete variant and projects only the chart-bearing field.
A `layer` accepts unit children or nested layers, which flatten in schema order;
it cannot directly contain `hconcat` or `vconcat`. Concat grids may nest.
Channels are deduplicated and emitted in canonical order. Scale source key
`kind` is emitted as language-neutral key `type`; accepted scale kinds are
`linear`, `log`, `symlog`, `sqrt`, `time`, `utc`, `ordinal`, `band`, `point`,
and `identity`.

`@LibertasChartScale` accepts `id,kind,min,max,mid,zero,reverse`; it has no guide
property. Canonical generated DateOnly `min` and `max` must be exact valid
Gregorian `yyyyMMdd` values; an equivalent source numeric spelling is
normalized before that check, and clients map the result by calendar-day
ordinal. DateTime bounds must be integral epoch seconds in
`0..253402214399`, and TimeOnly bounds must lie in
`0..86400` inclusive. Numeric scale properties use one shortest
round-tripping binary64 spelling. The effective target/scalar/scale-kind matrix
is closed; offsets and facets are band-only, and a numeric ordinal category
requires an integral, non-duration schema number. `@LibertasChartGuide`
requires `target` and `source`, followed by
optional `position,title,domain,ticks,labels,grid`. Sources are `scale`,
`value`, `span`, or `none`; positions are `auto/top/right/bottom/left`;
component values are `auto` or `none`. A scale/none guide is placed on the
field owning its
matching primary target. A value/span guide is placed on a guide-label field
with no scale-bearing channel. The following non-copyable field fragment shows
only the guide syntax; place an equivalent field inside a fully documented
named type:

```typescript
/**
 * Observation day
 * Identifies the calendar day represented by this group of observations.
 * @LibertasDateOnly
 * @LibertasChartChannel(tooltip)
 * @LibertasChartGuide(target=x, source=span, position=bottom,
 *                     title=none, grid=none)
 */
bucketDate: number;
```

`value` targets `x`, `y`, `theta`, or `radius`; `span` requires continuous
Cartesian `x,x2` or `y,y2`. Non-identity primary scales infer a scale-sourced
guide, while identity and offset scales infer none. One explicit declaration
for a target replaces the implicit guide. The exact target/side/component
matrix, layer and facet deduplication, localized titles and typed labels, tick
algorithm, span grouping, and limits are defined by
[Client-side charting](../charts/2d.md#64-chartguide-generic-guides).
Guide overlay is deterministic runtime policy rather than author metadata:
measure outside first, then overlay every eligible left/right guide together
when the post-guide plot would be narrower than 200 logical points. The exact
tap, accessibility, and chart failure presentation is defined by that
contract; clients do not choose their own breakpoints.
Preview uses a 240-point chart viewport with a 180-point charting region.
Fullscreen uses 75% of current usable height, capped in portrait by current
usable width. Complete vertical legend overflow remains in the one outer chart
scroll, whose one-shot initial position exposes the first charting region.
These are runtime policies and add no `ChartGuide` property.
Discrete positional `band` and `point` domains remain complete. A client uses
the contract's 20-point minimum leaf-category pitch and bounded X/Y category
viewports instead of growing the chart or dropping categories. Heatmaps can pan
both overflowing dimensions; a one-dimensional category chart consumes only
that dimension. This is runtime presentation and adds no decorator.
Dense monotone Cartesian lines and vertical areas are automatically resampled
per logical plot point and disclose that state with the noninteractive `≈`
indicator. Source rows remain exact for domains, tooltips, requests, and the
ordinary data view. Libertas imposes no chart-specific hard limit on row,
primitive, or source-path counts; structural/metadata limits and bounded
visible work remain enforced.

App task code may perform any data transformation, including time projection,
before returning the snapshot. Use native DateTime/DateOnly positions when the
automatic temporal scale is wanted, or emit transformed numeric coordinates
and attach typed `source=value`/`source=span` guide labels. The client localizes
those labels and never reverse-engineers the task's transform.

After parser extraction and dependency materialization, Studio's shared chart
validator rejects duplicate/misplaced metadata, invalid chart shapes,
incompatible scalar/channel and scale combinations, missing mark geometry,
secondary endpoints without primaries, and conflicting shared scale IDs.
Nearest same-name attribute lookup is a complete replacement, not a merge.
`Enumeration`, `LibertasDevice`, `LibertasVirtualDevice`, `LibertasUser`,
`LibertasEndpoint`, and `LibertasLanDevice` are categorical values whose raw
member/reference remains chart identity while the client resolves a localized,
permission-aware display name. `Action` is reserved and unsupported.
Formatted `Uint8Array` output may serve categorical or textual roles, but never
a quantitative role, continuous domain, or identity scale.
A visible plain `string` asserts genuine literal domain/user data such as a
proper name or free-form annotation. Never use one for a fixed App vocabulary,
dynamic App-authored wording, or a copied Libertas object name: use an enum,
formatted output, or the corresponding typed Libertas object reference. This
applies to every label, legend, facet, tooltip, value/span guide, and
accessibility path; key-only and guide-less identity appearance strings are
non-textual exceptions.
See [Client-side charting](../charts/2d.md) for the complete channel,
scale, geometry, composition, and fallback contract.

### 3D chart source attributes

Three-dimensional presentation uses five separate decorators:
`@LibertasChart3D`, `@LibertasChart3DChannel`, `@LibertasChart3DScale`,
`@LibertasChart3DGuide`, and `@LibertasChart3DView`. They add only schema
metadata and do not alter the typed response payload or the existing 2D
decorators.

```typescript
/**
 * Surface observation
 * Describes one measured height at a two-dimensional position.
 */
export interface SurfaceSample {
    /**
     * East-west position
     * Gives the observation's position along the east-west direction.
     * @LibertasChart3DChannel(x)
     */
    x: number;

    /**
     * North-south position
     * Gives the observation's position along the north-south direction.
     * @LibertasChart3DChannel(y)
     */
    y: number;

    /**
     * Measured height
     * Gives the height recorded at this position.
     * @LibertasChart3DChannel(z, tooltip)
     * @LibertasChart3DScale(id=height, kind=linear)
     * @LibertasChart3DGuide(target=z, source=scale)
     */
    height: number;
}

/**
 * Measured surface
 * Shows how measured height varies across the observed area.
 * ----
 * Surface observation
 * Represents one measured height at one position in the observed area.
 * @LibertasResponse
 * @LibertasChart3D(surface)
 * @LibertasChart3DView(projection=perspective, azimuth=45,
 *                      elevation=30, zoom=1, fov=45, orbit=true)
 */
export type Surface = SurfaceSample[];
```

Primitive tokens are `point`, `line`, `bar`, `surface`, `wireframe`, and
`stem`; compositions are `layer`, `hconcat`, and `vconcat`. Channels are `x`,
`x2`, `y`, `y2`, `z`, `z2`, `u`, `v`, `color`, `fill`, `stroke`, `opacity`,
`fillOpacity`, `strokeOpacity`, `size`, `shape`, `strokeWidth`, `strokeDash`,
`tooltip`, `detail`, `order`, `key`, `row`, and `column`. Scale syntax mirrors
`@LibertasChartScale`, including source `kind` becoming schema `type` that is
compiled into the [Display Template](../presentation.md#display-template), and adds
`logBase > 1` for a logarithmic scale, non-negative `growMin/growMax` fractions
for automatic continuous x/y/z domain growth, and inclusive `[0,1]` `padding`
for a `bar` base axis. Guides add `tickCount` and
`tickStep`, and may target `detail`.

Chart-family names are not extra tokens: scatter and point clouds use `point`,
bubbles add `size`, traces and waterfalls use `line`, columns use `bar`, height
maps and sampled mathematical grids use `surface`, contours layer `line` over
`surface`, and impulses use `stem`. Use `detail` for homogeneous multiple
series and `layer` for heterogeneous series.

`bar` accepts discrete or sparse continuous x/y bases with a continuous z
value. `x2`, `y2`, and `z2` may appear independently; omitted x/y extents and
the z baseline are inferred, while all six coordinates form an explicit
cuboid. The legacy any-orientation form remains valid when exactly two primary
axes are band scales and the sole continuous primary is the value axis. Surface
and wireframe `u,v` topology values may be arbitrary finite values and may
contain holes.

`@LibertasChart3DView` accepts, in canonical order, `projection`, `azimuth`,
`elevation`, `zoom`, `fov`, `orbit`, `aspectX`, `aspectY`, `aspectZ`, `targetX`,
`targetY`, `targetZ`, `pan`, and `zoomEnabled`. Projection is `perspective` or
`orthographic`; `orbit`, `pan`, and `zoomEnabled` are booleans; azimuth,
elevation, and target coordinates are finite; zoom and each aspect are greater
than zero; and `0 < fov < 180`.
Orthographic projection accepts `fov` but ignores it. No fixed Chart3D schema
cap limits rows, vertices, composition units, guides, or tooltips; normal
transport and runtime resource admission still applies. The complete schema
and rendering contract is
[Client-side 3D charting](../charts/3d.md).

`zoomEnabled=false` disables user zoom independently of orbit and pan without
changing the authored initial `zoom`.

These booleans independently gate the client's fixed camera actions; they do
not select or remap touch gestures. A preview is gesture-neutral, and its
**Explore 3D chart** action opens full-screen Chart Explorer, where one-finger
drag rotates, two-finger translation moves the view, and pinch/spread zooms.
There is no `mode`, gesture mapping, sensitivity, or control-label decorator or
runtime field. Tap-only controls and accessibility actions name Rotate, Move,
Zoom, and Reset explicitly. See the
[universal 3D interaction contract](../charts/3d.md#12-interaction-accessibility-and-fallback).

Point and stem-tip shapes use
`circle,square,triangle,diamond,sphere,cube,pyramid,cylinder` in that automatic
order; bars use `box,cylinder,pyramid,sphere`. Size zero suppresses or
degenerates the corresponding geometry without deleting its source row.

## Protocol and persistent-data schema evolution

Published protocol and persistent-data Unions are append-only contracts. Keep
each container's native name versionless and invariant for the App's lifetime,
and append new payload variants to that same Union. Do not replace it with a
version-named Union. See the
[App data-type design guidelines](../data-type-evolution.md).

## Endpoint Protocols

When defining endpoints for a Libertas AI Agent, you use decorators to define conversational roles and context:

- **`@LibertasEndpointSchema(MySchemaUnion)`**: Applied to a `LibertasEndpoint` field to define the payload structure for the agent. The target schema **must** be a union type.
- **`@LibertasEndpointServer`**: Marks the endpoint as a server.
- **`@LibertasDefault("developer-package::function")`** on an Endpoint client:
  when no schema-compatible server Endpoint exists, recommends provisioning the
  named task process. It does not filter existing compatible choices. If this
  Endpoint is absolutely required by its ancestor path, the client blocks task
  creation and requires that process to be installed when no compatible server
  exists.
- **`@LibertasFixed("developer-package::function")`** on an Endpoint client:
  after schema compatibility is evaluated, restricts choices to server
  Endpoints owned by that task process and forces its package into finalized
  `AppDependencies`. If an absolutely required occurrence has no matching
  server, the client requires that process to be installed before task
  creation.
  `Default` and `Fixed` are mutually exclusive here. Both Endpoint-ID selectors
  remain editable and may offer multiple server instances. The established
  separator is `::`; the value is task-process metadata, not the encoded
  Endpoint ID and not a schema-type FQN.

- **`@LibertasExcludeUI`**: On an Endpoint server carrying a resolved protocol
  schema, makes the Hub Control service omit that Endpoint from its UI-device
  list and LLM tool collection. On a reachable `Request` or
  `SubscriptionRequest` variant, removes that operation from generated Control
  UI and Hub LLM tool-definition generation. The admin plane still presents
  and persists the complete schema; the endpoint remains registered and
  protocol, routing, subscription, authorization, and wire behavior are
  unchanged.
- **`@LibertasProtocolConformance("VariantA & (VariantB | !VariantC)")`**: Declares variants required from the opposite endpoint when this Union is used as a protocol. It is valid on a Union declaration because the declaration alone does not identify protocol usage or a client/server role. Use direct Union variant native names with AND (`&`), OR (`|`), NOT (`!`), and XOR (`^`). During configuration, the client and server expressions are evaluated against each other's variants and both must pass. A scalar operand evaluates name presence; a Union-typed variant received from a Rust schema evaluates its nested conformance instead. Without the decorator, every direct variant declared by that Union context is mandatory. Shared variants must also have pass-through-compatible payloads. Root variants may be reordered because the client task host remaps their first Avro discriminator; positional fields and overlapping nested Union slots must remain aligned, and the receiving App decoder validates the untouched payload.
- **`@LibertasEndpointBaseObjects("$.device,$.room")`**: Filters the base objects context for the agent using one positional quoted comma-delimited path string. If multiple endpoints in a task resolve to identical base object scopes, compilation is blocked.

An occurrence is absolutely required when neither the Endpoint nor an ancestor
is `Nullable` and no ancestor array permits zero items. Clients traverse the
complete effective function-input tree, queue the original creation while
installing every missing required task, and rerun the check after installation.
Cancelling any required dependency installation cancels the originating task
creation. See the normative
[task pre-creation dependency check](../schema.md#62-task-pre-creation-static-dependency-check).

### Protocol Roles & Recursive Variant Validation
Within your protocol union type, every variant must satisfy one of the following requirements:
- **`@LibertasRequest`** / **`@LibertasResponse`**
- **`@LibertasSubscriptionRequest`** / **`@LibertasSubscriptionData`**
*(Note: A variant can hold multiple roles).*

Add the no-argument `@LibertasError` marker beside `@LibertasResponse` when
the complete response represents an error result rather than a normal result.
It emits `Error=true`; it does not imply the `Response` role and does not alter
the payload, discriminator, wire encoding, routing, or transition graph. The
payload remains an ordinary typed response. A consuming client may use the
marker to select its standard error procedure. The error is out-of-band of the
current protocol state transition: Android and iOS consume the correlated
response without installing or visualizing it, preserve the current
interaction and current response data, and leave retry available. They toast
the first renderable `FormattedText` found in deterministic depth-first
response order (including hidden fields), or a localized generalized error
when none can be rendered. Error classification takes precedence over normal
default-response, chart, and data presentation.

Outside workflows and automatic WorkflowRequest entry, a normal correlated
`@LibertasResponse` may omit its effective
`@LibertasNextRequest`. Omission is a non-transition acknowledgement, commonly
used by shared `Success` or `Failure` variants; it never enables every Request
in the Union. Clients complete the in-flight attempt without displaying the
acknowledgement as protocol data, changing the current choices, or displacing
a live DataReport. Its reference context follows the
[interaction reference-context rule](../protocols.md#interaction-reference-context).
For a variant that is both Response and SubscriptionData, the explicit runtime operation controls:
response delivery follows this rule, while SubscriptionData/DataReport delivery
still refreshes subscribed state. `@LibertasError` remains the higher-priority
out-of-band procedure above.

The [protocol contract](../protocols.md#default-and-explicit-subscriptions)
owns automatic versus explicit subscription selection, callback prerequisites,
and per-user access. Argument-free does not mean automatic.

Use `@LibertasAccessPrivilege("Read")`,
`@LibertasAccessPrivilege("Write")`, or
`@LibertasAccessPrivilege("Read,Write")` on a `@LibertasRequest` or
`@LibertasSubscriptionRequest` to restrict
which end users may invoke that runtime operation. The parser emits the
language-neutral integer bitmap `AccessPrivilege=1`, `2`, or `3`. The listed
levels are alternatives: any matching effective access to the owning endpoint
authorizes the operation. If omitted, the default is `Read,Write` (`3`), so a
read-only user can invoke the operation. Libertas cannot infer whether an
application-specific operation mutates state; developers must explicitly select
`Write` for operations that require write access. Studio filters disallowed
operations and the Hub independently decodes and enforces the selected protocol input
before routing it.

`AccessPrivilege` is not a task `Permission`. Permissions grant a task process
access to data and resources and are checked during configuration.
`AccessPrivilege` constrains an end user's access to one protocol operation at
runtime. Currently only `Read=1` and `Write=2` are defined; additional bitmap
values are reserved.

```typescript
/**
 * Check equipment status
 * Lets an authorized person view the equipment's current condition.
 * @LibertasRequest
 * @LibertasAccessPrivilege("Read")
 */
export interface GetStatus { kind: 0; }

/**
 * Change equipment availability
 * Lets an authorized person make the equipment available or unavailable.
 * @LibertasRequest
 * @LibertasAccessPrivilege("Write")
 */
export interface SetEnabled {
    kind: 1;

    /**
     * Equipment available
     * When selected, the equipment may operate; when cleared, the equipment
     * remains unavailable for operation.
     */
    enabled: boolean;
}
```

If the variant is directly resolved as a union type itself or a single-field union wrapper, role validation is performed recursively on the members of that union type.


`@LibertasWorkflowRequest("EditProfile")` declares the
[WorkflowRequest semi-role](../protocols.md#workflowrequest-semi-role).
`@LibertasPrevRequest("ReopenProfile,ReopenAddress")` belongs on a Request and
declares ordinary Back actions constructed from the current editor. Their
controls are suppressed in embedded master-detail trailing panes under the
[pane contract](../presentation.md#master-detail-panes).
NextResponse also permits Request variants returned as
correlated EditRequest messages; they need no Response role. See the
[workflow contract](../protocols.md#workflow-interactions) for completion,
initialization, explicit submission, and operation ownership.

`@LibertasLink("OpenTopic")` on any field, or on a struct, enum, or union
declaration, names the one Request variant that tapping that node opens;
see [links](../protocols.md#links) and [link rendering](../presentation.md#link-rendering).

`@LibertasLinkOnly`, with no argument, on a Request variant that a link
targets offers it only through links, never in Actions; see
[link-only requests](../protocols.md#link-only).

`@LibertasRequestConfirmation`, with no argument, on a Request variant
makes a client ask the person to confirm before it sends the request; see
[critical requests](../protocols.md#request-confirmation).

### Advanced Chaining & Caching
- **`@LibertasNextRequest("...")`** / **`@LibertasNextResponse("...")`**: Connects multi-step conversations by specifying the next valid step. The value **must be non-empty and match the native name of variants** (which is the resolved discriminator value name). You can provide a comma-delimited list for multiple valid next steps. `NextRequest` is optional on a Response; omission is the non-transition acknowledgement above, not an implicit list of every Request.

### State Management
- **`@LibertasCalculationRequest("CalculatePrice,CalculateShipping")`**: Field-only declaration of multiple calculation Request variants. Studio checks that their input `CopyFrom` mappings and participating fields agree in both directions. See the [field calculation contract](../protocols.md#field-calculation-requests) for current-request context, validity, and client implementation scope.
- **`@LibertasCopyTo("$.orderTotal")`**: Assigns a successful calculation response field into the originating editor. On a reduced SubscriptionData variant, use a strict variant-rooted destination list such as `^<FullState>,^<OtherState>` under the [reduced subscription pattern](../protocols.md#reduced-subscriptiondata-design-pattern). See [calculation results](../protocols.md#applying-calculation-results) for types, null, atomicity, indexed paths, ownership, resets, and deferred execution.
- **`@LibertasCopyFrom("$.first,$.fallback")`**: Valid only on protocol Request/SubscriptionRequest data. It uses one positional quoted comma-delimited path string. The [interaction reference-context rule](../protocols.md#interaction-reference-context) supplies the ordinary construction source; ordered selection, missing values, and initialization follow the [shared copy rules](../schema.md#shared-copyfrom-and-copyto-rules).
  - Path syntax and context follow [Libertas Schema Path](../schema.md#context-and-syntax).
    Ordinary CopyFrom supports strict variant-based paths such as
    `^<FullState.value>,^<OtherState.value>`, using the same resolution as directed
    subscription CopyTo. CopyFrom takes the first match; CopyTo rejects multiple
    matches as an App design flaw. `$` root-relative shorthand remains supported.
    Calculation mappings retain their separately defined invoking-field context.

### Semantic validation rules

Use `@LibertasValidationRules("...")` on a function, any data field, or any
schema data-type declaration. Function rules see all arguments as one Table;
field and named-type rules are cumulative and are rooted at the annotated value.
The empty path `[]` denotes that value. A square path reads current editor data,
while `{$.path,$.fallback}` searches the current interaction reference context
in source-then-path order under the [curly-reference Libertas Schema Path form](../schema.md#availability-null-and-interaction-ownership). Missing
and null paths are skipped; the first non-null value wins, and no match
evaluates to `null`.

Single `|` is a typed first-non-null fallback. It evaluates operands from left
to right, returns the first non-null value unchanged, and stops before later
operands. `false`, zero, empty text/collections, and objects are non-null and
win. It binds after additive operators and before `is` and comparisons:

```typescript
/**
 * @LibertasValidationRules('([preferred] | [legacy] | 100) >= 0 : "VALID_LIMIT";')
 */
```

The selected value may have any supported intermediate type, but the complete
rule must be Boolean. Use an explicit comparison for non-Boolean values; there
is no general truthiness conversion.

```typescript
/**
 * Nonnegative amount
 * Describes an amount that may be zero or greater when supplied.
 * @LibertasValidationRules('[] == null || [] >= 0 : "NONNEGATIVE";')
 */
export type NonNegative = number;

/**
 * Percentage range
 * Describes a percentage that may be selected between zero and one hundred.
 */
export interface Range {
    /**
     * Percentage
     * The user enters a percentage from zero through one hundred.
     * @LibertasValidationRules('[] == null || [] <= 100 : "PERCENTAGE_MAXIMUM";')
     */
    percentage: NonNegative;
}

/**
 * Choose an operating range
 * The user chooses inclusive lower and upper limits for an operating range.
 * The application prevents the lower limit from exceeding the upper limit.
 * Developed by Example Developer (example_developer).
 * @LibertasValidationRules('[minimum] == null || [maximum] == null || [minimum] <= [maximum] : "INVALID_RANGE";')
 */
export declare function configure(
    /**
     * Lower limit
     * The user enters the smallest amount included in the operating range.
     */
    minimum: number,
    /**
     * Upper limit
     * The user enters the largest amount included in the operating range.
     */
    maximum: number,
): void;
```

Null tests are valid for non-nullable fields because an editor may not have user
input yet. Final submission still requires ordinary structural validity and all
rules to evaluate to true. Prefix a rule string with `FinalOnly=false;` to opt
that complete rule set into incremental validation; omission means `true`, and
every rule remains in final validation. The directive stays inside the
`ValidationRules` value and is not a separate schema attribute. Every
expression must be followed by `:`, a quoted package `StringResources` key,
and `;`. Studio rejects undeclared keys and empty default-locale messages.
Clients stop at the first failing applicable rule in declaration order and
display its localized message. A missing or blank installed message produces a
generic validation failure. See the normative
[rule language](../validation-rules.md).

For a response-dependent numeric or DateTime control, including an editable request, use `@LibertasRangeMin` and
`@LibertasRangeMax` with one value expression each:

```typescript
/**
 * @LibertasRangeMin('{$.limits.minimum}')
 * @LibertasRangeMax('{$.limits.maximum}')
 */
percentage: number;
```

The expression must produce a bound compatible with the control’s semantic type. A Boolean predicate
such as `[] <= 100` is invalid; use `100` to set that maximum on an ordinary
Number. Date controls require their respective semantic date types. The
[dynamic range contract](../validation-rules.md#31-dynamic-numeric-ranges)
defines static/runtime type rejection, compatible date values, first-response
absence, and enforcement in both the GUI and final validation.

### Endpoint Signaling and Liveness (non-normative summary)

Use the [endpoint signaling orientation](../endpoint-lifecycle.md)
and the installed `@libertas-ai/types` endpoint declarations before implementing
callbacks. The public SDK owns the complete contract. Its TypeScript liveness
entry point is `libertasEndpointPeerAlive(endpoint, peer?)`.

---

## Standard Data Schemas

Ordinary Data API failures stop the task in the Hub runtime without permanently
disabling it or automatically restarting it. A function may explicitly opt into
checked status-returning calls with `DataErrorStatus`. See the
[Data API failure contract](../protocols.md#persistence-failure-and-the-no-status-api)
for the flag, return values, consistency responsibility, and third-party `Enable` action.

Place `@LibertasDataErrorStatus` in the public task function's JSDoc to permit
`libertasDataTry…` calls. They return `DataResult<T>` with a numeric `DataStatus`;
only `Success` carries `value`. Existing `libertasData…` calls retain default
failure behavior, even on opted-in tasks.

Use `@LibertasDataSchema(Type)` on the public App function, matching Rust's
`#[libertas_data_schema(Type)]` association. The argument resolves the function's
local or external persistent-data Union and emits `FunctionDataSchema=Type`.
Studio synthesizes `LibertasData=true` on the referenced Union. Do not add a
standalone marker to the Union. See the
[schema annotation inventory](../annotations.md) and
[persistent-data contract](../protocols.md#persistent-data-schemas).

---

## Singleton tasks

Use `@LibertasSingleton` on an exported function when a Hub may contain at most
one task that instantiates that function. The marker is invalid on the package,
types, fields, and function parameters, and it cannot take arguments or appear
twice.

```typescript
/**
 * Coordinate irrigation
 * The irrigation system uses one coordination service so all watering
 * activities share a single coordinator. Developed by Example Developer
 * (example_developer).
 * @LibertasExport
 * @LibertasSingleton
 */
export function coordinateIrrigation(): void { }
```

The singleton identity is the App plus the function's native name; it is not a
package-wide restriction and it does not reset between App versions. Studio
disables a singleton function when another matching task is already present,
and the Hub authoritatively checks every create, retarget, and task-version
change. A task may update itself. If two selected versions disagree about the
marker, the function remains singleton when either version declares it.

---

## Permissions

Use `@LibertasPermissions(CONST_NAME)` on a function to declare the permissions it requires. The referenced declaration must be a `const` initialized with a string array literal; the parser reports an error if the constant is missing, mutable, not an array, or contains a non-string literal. Every permission must also be a key in the package `StringResources`, and the default-locale value must be a non-empty explanation of why the permission is needed. Because the constants are schema-only metadata, the compiler may eliminate them when the application does not otherwise use them.

```typescript
/// Local weather service
/// Uses the installation's location to provide weather for nearby activities.
/// @LibertasStringResources(WeatherStrings)

const WeatherPermissions = [
    "libertas.permission.ACCESS_FINE_LOCATION"
] as const;

const WeatherStrings = {
    "libertas.permission.ACCESS_FINE_LOCATION":
        "Use the installation's location to request the correct local weather forecast."
} as const;

/**
 * Start local weather service
 * The service uses the installation's location to retrieve the correct local
 * forecast for weather-aware activities. Developed by Example Developer
 * (example_developer).
 * @LibertasExport
 * @LibertasPermissions(WeatherPermissions)
 */
export function startWeatherService(): void { }
```

This produces `"Permissions": "libertas.permission.ACCESS_FINE_LOCATION"` on the function and stores the explanation under the same key in the package document's `StringResources`. Missing keys and blank explanations are parser errors.

The same decorator may be placed on a built-in Endpoint property marked `@LibertasEndpointServer`. Its permissions are downstream access requirements and do not need same-key reason resources. The Hub compares them with the selected client App task's function permissions once during task approval/configuration; it does not repeat the check for each endpoint message.

```typescript
/**
 * Weather service connection
 * Describes the connection through which weather activities are available.
 * @LibertasExport
 */
export interface WeatherServer {
    /**
     * Weather service
     * The user chooses the weather service used by this weather-aware activity.
     * @LibertasEndpointSchema(WeatherProtocol)
     * @LibertasEndpointServer
     * @LibertasPermissions(WeatherPermissions)
     */
    endpoint: LibertasEndpoint;
}
```

## Network host access

Use `@LibertasAccessHost("host1,host2")` on a function to declare the DNS
hostnames its task process needs to contact. Every exact hostname must be a key
in the package `StringResources`, and its non-empty default-locale value must
explain why the access is required.

```typescript
/// Weather-aware irrigation
/// Adjusts watering with observations supplied by an online weather service.
/// @LibertasStringResources(NetworkStrings)

const NetworkStrings = {
    "api.weather.example":
        "Download weather observations used to adjust the watering schedule."
} as const;

/**
 * Weather-aware irrigation
 * The irrigation controller retrieves recent weather observations and uses
 * them to adjust watering schedules. Developed by Example Developer
 * (example_developer).
 * @LibertasAccessHost("api.weather.example")
 */
export function irrigationController(): void { }
```

The value is a comma-delimited hostname list with no whitespace. Hostnames are
unique without regard to DNS letter case. Do not include a URL scheme, port,
path, wildcard, or IP address; those are not hostname identities in this
contract. DNS may resolve the declared name to different addresses over time,
while the resource key and localized access explanation remain stable.

Without this attribute the task has neither local nor external TCP/UDP access.
A concrete host grants both TCP and UDP to all of its resolved IPv4/IPv6
addresses. Use `{network_access_local}` or `{network_access_external}` only for
the corresponding broad class; both are reserved resource keys and still need
localized reasons. The Hub refreshes DNS at runtime and adds new answers
without removing older answers until the task process restarts. Changing the
declaration forces that restart. See [App network access](../network-access.md).

---

## String Resources

Declare one localized catalog for the whole package. Notifications, permission
and host-access reasons, data names, `FormattedText`, and formatted chart labels from every
function use the same stable package-local keys. The task already identifies
the App package, so encoded tuples do not carry a function/type owner or FQN.

```typescript
/// Garden sprinkler controls
/// Lets people start watering for configured garden areas.
/// @LibertasStringResources(AppStrings)

const AppStrings = {
    "MSG_SUCCESS": "The sprinkler started watering the selected zone.",
    "MSG_ERROR": "The sprinkler could not start watering: {0}"
} as const;

/**
 * Start zone watering
 * The user can start the sprinkler for one configured watering zone.
 * Developed by Example Developer (example_developer).
 * @LibertasExport
 */
export function triggerSprinkler(): void { }
```
These strings provide the default translations for the UI. Users and developers can then translate these strings into other languages within the platform.

For a `TypesOnly` package, the same package-level declaration is valid. The
referenced declaration must be one non-empty `const` object literal with unique,
non-empty keys and literal string values:

```typescript
/// Shared status messages
/// Provides reusable localized status messages for compatible products.
/// @LibertasTypesOnly
/// @LibertasStringResources(ProtocolStrings)

const ProtocolStrings = { READY: "The equipment is ready." } as const;

/**
 * Status message
 * Describes localized wording shown for a reported condition.
 */
export interface OutputText {
    /**
     * Current status
     * Gives people the localized status reported by the application.
     * @LibertasReadOnly
     * @LibertasFormattedText
     */
    status: Uint8Array;
}
```

The keys are emitted in `QWHAScriptPackageJson.Attributes`; localized values are
stored once in `PackageDocumentJson.StringResources`. `OutputText` has no
resource field or document table.

---

## Localized Documentation

Libertas supports full localization of your package documentation. If your primary language is English, you can create translated documentation files for your package.

**No manual alignment is required**: The IDE automatically updates the structure of all localized documentation files when you save changes to your TypeScript source code, guaranteeing your translations stay in sync.

`@LibertasFoldable` permits manual folding on a struct, union, or list type or field; `@LibertasFoldable(initially_open=false)` starts it collapsed. Both are compiled into the [Display Template](../presentation.md#display-template). See [folding structs and unions](../presentation.md#folding-structs-and-unions).
`@LibertasInlineChoices` on an enum or union declaration, or on a field of either type, shows its choices inline as radio controls and is compiled into the [Display Template](../presentation.md#display-template); see [inline choices](../presentation.md#inline-choices).

## Backend-constrained choices

`@LibertasConstrainedBy("$.allowed_choices")` constrains an enum or union request field using a hidden,
backend-provided integer list in the current request. See the
[complete contract](../protocols.md#backend-constrained-choices) for numeric identity,
initialization, calculated allowed sets, validation, and payload editing.


Default endpoint entry uses `@LibertasDefaultRequest` on a Request variant whose payload needs no user input (Nil, or only hidden/read-only Fixed or Default fields). Response-level ConstrainedBy filters NextRequest through a hidden integer array. See [default entry and dynamic actions](../protocols.md#default-request-entry-and-dynamic-actions).

For additional response controls, put `@LibertasActionControl("sections")`
and `@LibertasHidden` on a direct integer-list field of a Response or
SubscriptionData payload with `NextRequest`. Give the list member its normal
integer NumberType annotation. The values are original protocol Union Request
discriminants; `"Actions"` names the built-in button. See
[named action controls](../protocols.md#named-action-controls).

For large workflow request payloads, `@LibertasCacheIdentity` marks direct root-record
identity fields for opt-in draft recovery. Small requests omit it. See the
[CacheIdentity contract](../annotations.md#cacheidentity) for immutable multi-field
identity, optional application protocol revision, and restoration rules.

## Subscription array append

On a report's root array field:

```typescript
/** @LibertasAppendTo("Entries.entries") @LibertasAppendIndex("$.start_index") */
added_entries: Entry[];
```

`AppendTo` names one working variant and root array. Optional `AppendIndex`
references the report's non-null integer field holding the expected current
length. Elements append in order, retaining duplicates and assignable nulls;
invalid indices or resulting values reject the entire report. The App supplies
complete initial snapshots and owns ordering and retention. See the
[array append contract](../protocols.md#subscriptiondata-array-append) for
placement, combining CopyTo with append, exclusion of index controls from copied
data, writer conflicts, independent recipients, and live charts.

## AI Search guidance

Use `@LibertasAINavigationPrompt("NAVIGATION_PROMPT")` on a protocol Union or page variant to reference
a package StringResource. Both Union and page guidance contribute, in that
order. Document destination vocabulary in each offered topic or Enumeration
member’s extracted description. See [AI Search](../protocols.md#ai-navigation-and-ai-search)
for candidate construction, selection, confirmation, and submission rules.
