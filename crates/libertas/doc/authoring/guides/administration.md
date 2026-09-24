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
5. [Tasks Tab](#3-tasks-tab)
6. [Groups Tab](#4-groups-tab)
7. [Libertas Plaza Tab](#5-libertas-plaza-tab)
8. [Security Tab (Users & Groups)](#6-security-tab-users--groups)
9. [Access Control List (ACL) Tab](#7-access-control-list-acl-tab)
10. [AI Connectors Tab](#8-ai-connectors-tab)
11. [Troubleshooting & Best Practices](#troubleshooting--best-practices)

---

## Getting Started

### Accessing the Admin Panel
To open the Admin Panel in your active VSCode workspace:
1. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS) to open the VSCode Command Palette.
2. Type **`Libertas: Open Admin Panel`** and select it.
3. The panel will load and establish a connection to your active local or remote Libertas Hub runtime.

> [!NOTE]
> Ensure the Hub is reachable and your Studio client is authorized. Studio does not commission a new or reset Hub; use the mobile app for that first.

---

## Interface Overview
The Admin Panel features a management layout with:
- **Left/Top Navigation sidebar** for selecting the active management category.
- **Dynamic cards** presenting metrics, statuses, and action logs.
- **Searchable list views** with real-time query filtering.
- **Tree-view selector dropdowns and modal search choosers** for hierarchical entities.

---

## 1. My Hub Tab
The **My Hub** tab is the landing page and provides hardware status, operating system stats, and basic control operations for the connected Hub.

### Information Sections
- **Hub Information**: Details the Hub's custom friendly name, IP address on the network, Client ID, active User ID, Hub Type (**Local Hub** or **Cloud Hub**), and runtime Platform (e.g., Libertas OS).
- **Hub Settings** and **Advanced** (super users): **Connection Settings** and **Reconfigure Thread** configure a Local Hub's home network and Thread radio, so the panel omits them for a Cloud Hub.
- **Firmware & Runtime Details**: Displays the kernel version, active firmware version, and the internal IP address.
- **Operations & Management**:
  - **Reboot Hub (Soft)**: Reboots the Libertas software runtime daemon without power cycling the device.
  - **Reboot Hub (Hard)**: Initiates a complete operating system reboot on the hub hardware.
  - **Halt Hub (Shutdown)**: Shuts down the hub processor safely. Use before disconnecting power.
  - **Reset Hub (Setup State)**: Resets the hub configuration back to factory setup states.
  - **Refresh Data**: Reloads configurations, device trees, and task tables.

> [!WARNING]
> **Reset Hub** will erase current local settings and user configurations. Only trigger this operation if you intend to re-provision the hub.

---

## 2. Devices Tab
The **Devices** tab shows physical radio transceivers, network cameras, and IoT devices registered with the hub.

### Device Categories
- **Zigbee Devices**: Displays connected Zigbee endpoints (e.g., smart bulbs, motion sensors, relays). Shows the friendly name/SKU, the IEEE MAC address, short 16-bit address, and connection state (Online/Offline).
- **Matter Devices**: Displays connected Thread or Wi-Fi Matter devices. Shows Node IDs and statuses.
- **ONVIF Cameras**: Lists network-connected cameras. Shows IP address, camera model/data, and state.

### Actions
- **Remove**: De-registers the physical device from the hub controller network.

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
- **Task Cards**: Shows developer metadata, package name, executing function, and unique Task ID.
- **Active Task Settings**:
  - **Version Selector**: Dynamically upgrades or downgrades the task's script package version via a dropdown.
  - **Toggle Slider**: Toggle the running task's status state (Enabled/Disabled).
  - **Edit**: Edit virtual endpoints, virtual device names, or configuration parameters.
  - **Stop / Start**: Directly control execution without removing task definitions.
  - **Remove**: Destroys the virtual task instance.

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
Displays a list of registered users.
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
The **ACL** tab allows fine-grained security policies by restricting access to specific objects (Devices, Tasks, Apps, and other Users).

The App catalog includes an App when the user has any effective permission on
it. **Configure** is specifically required before a user with the global
**Add/Remove Tasks** privilege may create a task using that App. Catalog
visibility does not authorize task creation. When an existing task is retargeted
to another App, that new App also requires Configure access.

### Toggle View Modes
1. **By User / Group**
   - Select a target User or Group using the dropdown or search dialog chooser.
   - Shows all resource object permissions assigned to them.
   - Includes quick filtering input to search current access paths.
   - **Add Object Permission**: Define access permissions (Allow, Deny, etc.) to target devices, apps, or tasks.

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
The **AI** tab configures the LLM engines driving the Hub's natural language agent features.

### Features
- **Add Connector**: Create an LLM service connector definition.
- **Connector Configuration**:
  - **Model ID**: Specifies the target model.
  - **Owner**: Identifies the system user that owns the api integrations.
  - **Configuration Snippet**: JSON editor block specifying API keys, base endpoints, temperature, and custom parameters.
- **Actions**: Edit connector configurations or Delete connector keys safely.

---

## Troubleshooting & Best Practices

### The Admin Panel is Blank or Displays "Loading configurations..."
1. Check that Libertas Studio is active and the intended Hub is selected.
2. Use **Test Connection** in Hub Management and verify the address and network.
3. If authorization has expired or been revoked, reconnect through Studio's QR
   authorization flow with an authenticated mobile app. Do not manually edit
   credentials. If the Hub was reset, commission it in the mobile app first.
4. Confirm the account has permission for the operation. Refresh the panel after
   connection or permission changes.
