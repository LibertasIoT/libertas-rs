# Understand the platform

Concept explanations connect the steps in the guides to the behavior you see.
Use them when you understand what to click but want to know why the workflow
works that way.

## Apps and tasks

An App package is reusable software and its public types. A public function
can be configured as a task. Several tasks can use the same function with
different settings unless the function has a singleton restriction.

## Source, schema, and visible text

Source defines the App. Studio extracts the schema used to validate and
present values, and localized documents supply human-readable text. Changing
a translation should not change the schema's structure or runtime behavior.

## Actions and ongoing information

A task can expose operations and report data. A response belongs to an
interaction; a subscription provides ongoing information. Keeping these roles
clear helps explain why a confirmation can leave current data visible and why
live data may change without a new button press.

## Shared contracts

Two Apps can cooperate only when their types and protocols remain compatible.
Public identities, variant order, and supported capabilities matter across
versions. The [shared reference](../reference/index.md) is the common home for
those exact rules; [human guides](../guides/index.md) show how to use them.

[Documentation home](../index.md)
