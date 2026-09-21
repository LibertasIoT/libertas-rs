# Using Libertas Studio

Studio helps you take App source to a running Hub, inspect its generated
interface, and diagnose deployment. Start with a commissioned Hub and a
TypeScript or Rust project. Studio requires Hub service version
`20260803000000` or later and rejects obsolete login/session message flows.
Legacy Hubs are supported only by the separate Android/iOS compatibility paths. [Getting started](getting-started.md) explains Apps,
tasks, and clients; [developing Apps](developing-apps.md) introduces source work.

![Studio prepares an App, deploys it to a Hub, and clients configure tasks and use their controls.](../assets/diagrams/platform-overview.svg)

The Hub runs the task. Studio's deployment prepares the software that a task
will use; deployment alone does not create a configured task.

## Using the schema compiler with Codex

You can check App schemas before connecting a Hub. Install Node.js so `node`
is available on your PATH, install the project's dependencies, and open Studio.

1. Run **Libertas: Configure Schema Compiler for Codex** from the Command Palette.
   This registers Studio's compiler in your Codex user configuration, preserving
   unrelated settings. It applies to Codex CLI and its IDE extension.
2. Restart Codex or its IDE extension to load the `compile_schema` tool.
3. Ask Codex to edit the App and validate its saved source. For example:
   “Add the requested configuration field, then use compile_schema to fix any
   schema errors and run the App's tests.”

The server supplies the validation workflow to Codex automatically. Request
artifacts when you want to inspect the root and all dependent schemas; ordinary
checks return compact diagnostics. See the
[schema compilation contract](../reference/app-contract.md#schema-compilation-with-codex)
for inputs, dependency completeness, and validation limits. After updating
Studio, activate it once to refresh the registered compiler launcher.

## Managing Libertas Hubs

To run, debug, or deploy your TypeScript or Rust Apps to the platform, you must connect your VS Code environment to one or more **Libertas Hubs**. Hub commissioning is never performed by VS Code: complete it first in the Libertas mobile app. Hub connections are managed through the **Libertas Activity Bar** and sidebar webview in VS Code.

### 1. Connecting a Commissioned Hub

Connecting a Hub triggers a secure QR authorization process that registers VS Code as a client without exposing raw user credentials. The extension refuses setup and recovery-state Hubs before requesting QR authorization.

1. Open the **Libertas View** in the VS Code Activity Bar.
2. In the **Hub Management** section, enter a friendly **Name** for the Hub and its **IP Address**.
3. Click **Connect Hub**. The extension will initiate the secure client-authorization sequence:
   * Studio checks that the Hub is commissioned and ready for user login.
   * Scan the displayed one-time QR code with a Libertas mobile app already
     authenticated to that Hub. The user who scans and approves becomes the
     Studio client's user; Studio does not ask you to select another user.
   * Studio stores the resulting credential in VS Code Secret Storage. Never
     copy credentials into project files or paste them into an Admin form.

---

### 2. Testing Hub Connection

You can check if a registered Hub is online and has maintained its secure connection status:
1. Locate the Hub in the sidebar list.
2. Click **Test Connection** (or the test icon next to the Hub).
3. The extension performs a TLS handshake and verifies that the remote certificate fingerprint matches the recorded secure signature. A warning is shown if there is a hash mismatch (indicating the Hub was reset or there is an active man-in-the-middle event).

---

### 3. Deleting a Hub

If you no longer need to develop against a specific Hub:
1. Find the Hub in the sidebar list.
2. Click the **Delete** (trash) icon.
3. The extension purges the Hub configuration and its corresponding cryptographic tokens from the system's VS Code Secret Storage.

---

## Deploying to a Hub

Deployment prepares the selected TypeScript or Rust App, its schema, and its
localized documents for one Hub. Choose the Hub that should run the result;
publishing to Plaza is a separate workflow.

### 1. The Deployment Workflow

When you initiate a deployment using the `Libertas: Deploy to a Hub` command or the **Deploy** button in the Libertas Sidebar:

1. **Compilation Validation**:
   Studio compiles TypeScript projects locally (`tsc`) to verify there are no
   syntax or semantic errors. Rust applications are uploaded as source and built
   natively by the selected Hub.
2. **Local Schema Parsing**:
   Studio extracts the schema and default-locale documentation from TypeScript
   JSDoc or Rust source/rustdoc. The declared `DefaultLocale` applies; omission
   means `en`.
3. **Dependency preflight**: Review the complete dependency-first plan. It
   distinguishes deployable local projects, package-cache comparison inputs,
   and exact Plaza transitive releases. Missing source, incompatible schemas,
   cycles, unresolved exact releases, or conflicting pins stop before install.
4. **Dependency installation**: Studio installs and verifies the approved
   closure before submitting the local App. The
   [dependency contract](../reference/deployment.md) defines the exact checks,
   version rules, and read-back behavior. A failed dependency stops its dependents.
5. **Hub Metadata Synchronization & Patching**:
   * The extension connects to your selected Hub and fetches any existing metadata, schemas, and translated localized documents.
   * **Automatic Structural Patching**: If the app already exists on the Hub, the extension automatically patches existing localized translation files. It aligns translated text by native schema identity where possible and warns
     when changed structure needs review. See the
     [translation migration guarantees](../reference/translation.md#migration-guarantees).
6. **Finalizing with the Schema Editor**:
   The Schema Editor Panel opens in Deploy/Submit mode, allowing you to preview and finalize your metadata.
7. **Git-Friendly Local Workspace Sync**:
   When you click **Submit**, the extension synchronizes and saves all finalized localized translation files (`.doc.json`) directly back into your local workspace folder, so you can review and commit the changed files. Studio does not commit them for you.
8. **Upload and native prerequisite check**:
   Studio securely uploads the compiled schema signature and patched localized
   documents. For an executable Rust package, the Hub parses the uploaded
   `Cargo.toml` before admitting the build. If explicitly declared native build
   packages are missing, deployment automatically opens a modal containing the
   complete APT plan. **Install and Continue** applies that short-lived plan and
   resubmits the exact same source revision; closing or rejecting the modal ends
   the deployment and queues nothing. No separate package-management command is
   required for this flow. See
   [Build and deployment metadata](../reference/languages/rust.md#build-and-deployment-metadata)
   for declarations, target selectors, permissions, and the distinction between
   build and runtime packages.

---


## Schema Editor

A Hub-loaded Schema Editor reads the selected Hub's schema and documents. A
local editor reads workspace source and `.doc.json` files. Confirm which mode
and package you opened before editing. Referenced packages can be read-only.

The default document comes from source; correct its labels and descriptions
there. The [translation guide](translation.md) owns locale selection, Codex and
manual handoffs, immediate import persistence, and staged removal. Imported
translations in a Hub-connected editor upload immediately, without **OK**.

## Logs and source editing

Use **Libertas: Monitor Hub** or **Monitor** to view streaming execution logs.
The log list scrolls while its controls remain visible and presents connection
and session status in readable form. A connection failure is distinct from an
App build or runtime failure; see [troubleshooting](troubleshooting.md).

In TypeScript comments, typing `@Libertas` offers completion descriptions and
parameter snippets. Libertas annotations receive syntax highlighting. The
[source annotation reference](../reference/annotations.md) owns supported
spellings and arguments; completion is an editing aid, not an additional API.

## Configure and use the deployed App

After deployment succeeds, use [Administration](administration.md) to create
and edit tasks. Use [Task controls](control.md) for requests and live results.
Choose [Plaza](plaza.md) when you want to publish the verified deployed targets.
