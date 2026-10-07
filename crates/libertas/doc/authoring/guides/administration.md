# Libertas Hub Admin Panel User Manual

The **Libertas Hub Admin Panel** in Studio manages the selected Hub's devices,
Apps, tasks, users, permissions, and AI connectors. Connect and authorize a
commissioned Hub through [Studio](studio.md#managing-libertas-hubs) first.
Controls available to you depend on your account's effective permissions.

---

## Table of Contents
1. [Getting Started](#getting-started)
2. [Interface Overview](#interface-overview)
3. [My Hub Tab](#1-my-hub-tab)
4. [Devices Tab](#2-devices-tab)
5. [HVAC Tab](#hvac-tab)
6. [Tasks Tab](#3-tasks-tab)
7. [Task Data Tab](#task-data-tab)
8. [Groups Tab](#4-groups-tab)
9. [Libertas Plaza Tab](#5-libertas-plaza-tab)
10. [Security Tab (Users & Groups)](#6-security-tab-users--groups)
11. [Access Control Tab](#7-access-control-list-acl-tab)
12. [AI / LLM Tab](#8-ai-connectors-tab)
13. [Troubleshooting & Best Practices](#troubleshooting--best-practices)

---

## Getting Started

### Accessing the Admin Panel
To open the Admin Panel in your active VSCode workspace:
1. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS) to open the VSCode Command Palette.
2. Type **`Libertas: Open Admin Panel`** and select it.
3. The panel will load and establish a connection to your active local or remote Libertas Hub runtime.

> [!NOTE]
> Ensure the Hub is reachable and your Studio client is authorized. Studio does not commission a new or reset Hub; use the mobile app for that first.

### Administrative access

**Allow Administration** permits an administrative connection; each operation
still requires its own permissions. Clients save the Hub's effective permission
flags and refresh them when the connected Hub reports a change or a new session
starts. The Hub remains authoritative for every operation.

If saved permissions deny administration, tapping the mobile admin icon or
opening Studio's Admin Panel opens Control. This connection refreshes the
saved permissions. If administration has since been granted, a later admin
action opens Admin using the updated flags. Use an administrative connection
for [Hub-assisted Plaza operations](../reference/deployment.md#plaza-installation-access).

On Android and iOS, saving a Control layout to a modern Hub and browsing its
full icon catalog also require **Allow Administration**. Without it, you can
read your assigned layouts, edit layouts locally, and download the icons
assigned to your visible devices, tasks, and layouts. Permission changes apply
while connected.

---

## Interface Overview
The Admin Panel features a management layout with:
- **Left/Top Navigation sidebar** for selecting the active management category:
  **My Hub**, **Devices**, **HVAC**, **Tasks**, **Task Data**, **Groups**,
  **Libertas Plaza**, **Security**, **Access Control**, and **AI / LLM**.
- **Dynamic cards** presenting metrics, statuses, and action logs.
- **Searchable list views** with real-time query filtering.
- **Tree-view selector dropdowns and modal search choosers** for hierarchical entities.

---

## 1. My Hub Tab
The **My Hub** tab is the landing page and provides hardware status, operating system stats, and basic control operations for the connected Hub.

### Information Sections
- **Hub Information**: Details the Hub's custom friendly name, IP address on the network, Client ID, active User ID and User Name, Hub Type (**Local Hub** or **Cloud Hub**), and runtime Platform (e.g., Libertas OS).
- **Firmware & Runtime Details**: Displays the kernel version, active firmware version, and the Hub's local IP address.
- **Hub Settings** (super users): **Connection Settings** (Local Hub only), **Location & Time Zone**, and **HVAC Systems**, which opens the HVAC tab.
- **Advanced** (super users): **Reconfigure Thread** (Local Hub only), **Hub Configuration**, **Developer Access**, and **Reset Hub**, which returns the Hub to its setup state.
- **System** (super users):
  - **Check for Upgrades**: Looks for a newer Hub release.
  - **Restart**: Restarts the Libertas software runtime without power cycling the device.
  - **Force Restart**: Reboots the Hub's operating system.
  - **Shut Down**: Shuts down the Hub safely. Use before disconnecting power.
  - **Refresh Data**: Reloads configurations, device trees, and task tables.
- Other users see a **Management** card that offers only **Refresh Data**; Hub
  settings and system operations require a super user.

> [!WARNING]
> **Reset Hub** will erase current local settings and user configurations. Only trigger this operation if you intend to re-provision the hub.

---

## 2. Devices Tab
The **Devices** tab shows physical radio transceivers, network cameras, and IoT devices registered with the hub.

All devices appear in one **Configured Radio & Media Devices** list. A badge
on each row identifies its kind:
- **Zigbee** devices (e.g., smart bulbs, motion sensors, relays) show the friendly name/SKU, the IEEE MAC address, short 16-bit address, and connection state (**Online** or **Offline**).
- **Matter** devices show their Node ID.
- **ONVIF Camera** rows show the camera's IP address.

### Actions
- **Remove**: After confirmation, de-registers the physical device from the Hub.

---

<a id="hvac-tab"></a>

## HVAC Tab
The **HVAC** tab lists the Hub's configured HVAC systems with their heating and
cooling types and stages. Super users and users with the Add/Remove Devices
privilege can add, edit, and remove systems.

---

## 3. Tasks Tab
Tasks are virtual running programs compiled from installed platform apps. This tab manages active execution threads.

### Interface Details
- **New Task** opens the App function chooser, then the configuration editor.
  Read [Installed and Search Plaza](plaza.md#browse-installed-apps-and-search-plaza)
  for row hierarchy, full descriptions, and Install/Upgrade/Select behavior.
  Packages without public functions are omitted from this chooser.
- Creating a task requires the access described in
  [App access control](#7-access-control-list-acl-tab). Seeing an App in the
  catalog alone does not authorize task creation.
- **Task Cards**: Show the Task ID, developer, package, executing function,
  and a status badge: **Active**, **Stopped**, or **Inactive**.
- **Active Task Settings**:
  - **Version dropdown** (**Version** *x*): Offers the task's current App
    version and the version installed on the Hub. Choosing the other one,
    after confirmation, moves the task to it; installing or upgrading an App
    never changes a task's version by itself. See
    [task version selection](../reference/app-contract.md#task-version-selection).
  - **Enable/Disable slider** and the **Stop** / **Start** button: The button
    reads Stop while the task is enabled and Start while it is disabled; both
    send the same enable or disable request.
  - **Edit**: Edit virtual endpoints, virtual device names, or configuration parameters.
  - **Remove**: After confirmation, stops and removes the task and its
    persistent data. See [task removal](../reference/app-contract.md#task-removal)
    for the effect on dependent tasks.
- A task that stops after a failed data read or write shows **Stopped**. The
  Hub keeps its enabled setting, so the slider still shows it enabled and the
  button reads **Stop**, but never restarts it automatically. After correcting
  the cause, choose **Stop** and then **Start** to start it again. See the
  [Data API failure contract](../reference/protocols.md#persistence-failure-and-the-no-status-api).

---

<a id="task-data-tab"></a>

## Task Data Tab
The **Task Data Explorer** surveys an App task's stored data. Choose the
**App task**, search record names, and open a record to inspect it.

---

## 4. Groups Tab
Device Groups handle direct device-to-device bindings.
- **Hornet Groups**: Shows the device association groups configured under the Hornet protocol.
- Displays Group ID (`LoadId`), routing protocol, and bound endpoints count.

---

## 5. Libertas Plaza Tab
The **Libertas Plaza** tab lists software packages installed on the selected Hub
and packages available through Plaza. Developer sign-in and sign-up are opened
as a separate center-editor page with **Libertas: Open Libertas Plaza**.

Use the [Plaza guide](plaza.md) for installed/search rows, developer setup,
repository selection, publication review, and target withdrawal.

### Installed Apps
The [shared catalog behavior](plaza.md#browse-installed-apps-and-search-plaza)
also applies here, including visible TypesOnly packages and full descriptions.
App details provide these additional controls:

- **Documents**: Expands a detailed documentation container in the detail view containing:
  - **Friendly Display Name & Description**: Outlines the developer's purpose for the app.
  - **Functions**: Detailed list of registered app endpoints, developer signatures, and sorted parameter lists.
  - **Types**: Custom data formats and fields parsed from the app schema.
  - **Raw Signature (Schema) JSON**: Collapsible details tag containing the exact schema definition code.
- **Translate**: Opens the app's translations for the selected Hub.
- **Uninstall**: Permanently removes the app package from the hub runtime storage.

### Available in Libertas Plaza
Use **Search Plaza** and the version-appropriate action described in the
[Plaza guide](plaza.md#browse-installed-apps-and-search-plaza).

---

## 6. Security Tab (Users & Groups)
The **Security** tab administers user authentication, administrative privileges, active sessions, and group hierarchies.

### System Users
Displays the registered users your administrative permissions allow you to see.
When the Hub has more than 1,024 nongroup accounts without administrative
admission, its initial user list omits all such accounts. The count covers the
whole Hub, including accounts outside your visible list. At 1,024 or fewer,
the initial list uses the ordinary permission rules.

Administrative admission requires effective **Login Allowed** and either
**Allow Administration** or **Super**. Groups and accounts with this admission
remain eligible for the initial list under your existing permissions. The
threshold affects only that initial list: group memberships, permission
records, explicit user operations and live updates retain their ordinary rules.

- **User Privileges Reference**:
  Each user or group can be assigned specific privilege bits that determine their access permissions. When a user belongs to one or more groups, they automatically inherit the privileges of those groups.

  | Privilege Badge | Description |
  | :--- | :--- |
  | `🔑 Login` | **Login Allowed**: Permission to authenticate and establish a session with the Hub. |
  | `👑 Admin` | **Super / Admin User**: Full superuser/administrative privileges across all sections of the Hub. |
  | `🔌 Devices` | **Add/Remove Devices**: Permission to pair, modify, or detach physical hardware devices. |
  | `📦 Apps` | **Add/Remove Apps**: Permission to upload, install, or uninstall Thing Apps. |
  | `⚙️ Tasks` | **Add/Remove Tasks**: Permission to create, configure, start/stop, or delete script automation tasks. |
  | `🎬 Scenes` | **Add/Remove Scenes**: Permission to create, trigger, or delete custom scene logic. |
  | `👤 Users` | **Add/Remove Users**: Permission to manage system user accounts, groups, and privileges. |

  > [!IMPORTANT]
  > **Privilege Resolution Logic & Login Privilege Restrictions**:
  > - **Additive Privileges**: Functional permissions (such as `Super / Admin`, `Devices`, `Apps`, `Tasks`, `Scenes`, and `Users`) are additive. If a user is assigned these privileges directly OR inherits them from any group they belong to, the privilege is enabled.
  > - **Restrictive Login Privilege**: The `Login Allowed`  privilege follows a restrictive **AND** logic. For a user to successfully authenticate and log into the Hub, login must be enabled on the user account **AND** on every group the user is a member of. If any group to which the user belongs has `Login Allowed` disabled, the user's login access is automatically revoked, even if it is enabled directly on their user account.
- **Actions**:
  - **Clients**: Fetches and lists active login sessions (VSCode, Android, Apple, etc.) with options to invalidate tokens.
  - **Privileges**: Assigns/revokes target feature privilege bits.
  - **Groups**: Manages group memberships for the user.
  - **Delete**: Removes the user (built-in system users are protected).

### System Groups
Groups group users together to inherit roles and access permissions.
- Shows direct member count and list of active members.
- Includes action modals to manage members, configure privileges, and delete group containers.

---

## 7. Access Control List (ACL) Tab
The **Access Control** tab allows fine-grained security policies by restricting access to specific objects (Devices, Tasks, Apps, and other Users).

The App catalog includes an App when the user has any effective permission on
it. **Configure** (shown as **Config**) is specifically required before a user with the global
**Add/Remove Tasks** privilege may create a task using that App. Catalog
visibility does not authorize task creation. When an existing task is retargeted
to another App, that new App also requires Configure access.

### Toggle View Modes
1. **By User / Group**
   - Select a target User or Group using the dropdown or search dialog chooser.
   - Shows all resource object permissions assigned to them.
   - Includes quick filtering input to search current access paths.
   - **Add Object Permission**: Grant **Read**, **Write**, **Config**, or **Remove** access to target devices, apps, or tasks. A super user automatically has Config and Remove, but Read and Write must still be granted.

2. **By Object**
   - Select the target resource category: **Devices**, **Apps**, **Tasks**, or **User / Group (Target)**.
   - Select the specific resource object using the dropdown or search dialog chooser (e.g., a physical light switch, task logical device, or unassigned scene).
   - Shows the list of **Users & Groups with Access**.
   - Identifies if access is **Direct** (assigned directly to the user) or **Inherited** (gained via group membership privileges).
   - **Add User/Group Access**: Grants access permissions to a new user or group for this object.

> [!TIP]
> If a user has unexpected access to an object, switch to the **By Object** view mode and verify if the user is inheriting the access from a system group they belong to.

---

## 8. AI Connectors Tab
The **AI / LLM** tab (**AI & LLM Connectors**) configures the OpenAI or Gemini
connections used by the Hub's AI features.

### Features
- **Add LLM Connector** opens a form with:
  - **Display Name**: The connector's name.
  - **Owner User**: The Hub user who owns the connector.
  - **Model ID / Provider**: A preset model or a custom model ID.
  - **API Key**: The provider credential, entered in its own field.
  - **Configuration (JSON)**: Additional provider parameters.
- **Actions**: Edit or delete a connector.

---

## Troubleshooting & Best Practices

### The Admin Panel is Blank or Displays "Loading configurations..."
1. Check that Libertas Studio is active and the intended Hub is selected.
2. Use the Hub's **Test TLS Connection** icon in the Studio sidebar and verify the address and network.
3. If authorization has expired or been revoked, reconnect through Studio's QR
   authorization flow with an authenticated mobile app. Do not manually edit
   credentials. If the Hub was reset, commission it in the mobile app first.
4. Confirm the account has permission for the operation. Refresh the panel after
   connection or permission changes.
