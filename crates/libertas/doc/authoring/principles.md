# Start here

Libertas documentation for people is deliberately short. This page gives the
few principles everything else follows and the words you need to ask good
questions. For anything more, ask an AI assistant to read the Libertas
documentation and answer your question; the prompts below point it at the
right entry.

## Principles

1. **Source is the complete App.** An App is an ordinary Rust or TypeScript
   library. Its data types, interactions, presentation hints, and
   documentation comments are written in the source; Studio extracts the
   schema from it. Generated artifacts are never edited by hand.
2. **One description serves everyone.** People, other software, and AI
   assistants use the same declared data and operations. There is no separate
   GUI, API, or AI definition to keep in step.
3. **The service decides every step.** A person's client sends one request at
   a time and waits for its reply; the App's reply decides what comes next.
   Clients keep no history of their own.
4. **The platform owns the rest.** Presentation, persistence, isolation,
   security, auditing, and automation come from the platform. An App describes
   meaning and behavior, not screens or storage plumbing. When storage fails,
   the Hub stops the task by default.
5. **Smallest logically complete design.** Every required behavior, reusing
   the platform's mechanisms, with nothing duplicated.
6. **Contracts are exact.** What the reference states is promised; what it
   does not state is not. A good answer quotes the rule it relies on.

## Words to know

| Term | Meaning |
| --- | --- |
| Hub | The device that runs Apps, stores their data, and connects your devices. |
| App | A package of reusable software and its public types, installed on a Hub. |
| Task | A configured, running use of one of an App's public functions. |
| Studio | The development environment: projects, validation, preview, deployment. |
| Client | Studio, or the Android or iOS app, used to operate tasks. |
| Endpoint | A point where a task offers interaction to people and other software. |
| Protocol | The closed set of an endpoint's Requests, Responses, and live data. |
| Request / Response | What a client sends, and the App's reply to it. |
| SubscriptionData | Live data a task publishes while you watch. |
| Workflow | A sequence of editable requests whose next step the App decides. |
| Display Template | Rules that arrange and style the generated interface, kept apart from the schema. |
| CodeBot | An AI coding agent that writes App source from the CodeBot documentation. |

## Ask an AI assistant

Copy the prompt that fits you into any AI assistant that can read web pages,
then add your question. Each prompt points at a different entry in the
public [Rust SDK documentation](https://github.com/LibertasIoT/libertas-rs).

**To have an AI build or change an App** (a CodeBot):

```text
Read https://github.com/LibertasIoT/libertas-rs/blob/main/crates/libertas/CODEBOT.md
and follow it. Build a Libertas App that:
```

**If you develop Apps** and want explanations:

```text
Read https://github.com/LibertasIoT/libertas-rs/blob/main/crates/libertas/DEVELOPER-ASSISTANT.md
and follow it to answer my questions about developing Libertas Apps.
My question:
```

**If you own a Hub or use Apps:**

```text
Read https://github.com/LibertasIoT/libertas-rs/blob/main/crates/libertas/USER-ASSISTANT.md
and follow it to answer my questions about using Libertas.
My question:
```

An answer should quote the rule it relies on and link its section. Treat an
answer without a source as unverified, and check it against the linked page.

## See it working

- [Developing Apps](guides/developing-apps.md): one tutorial from source to a
  running task.
- [Federal tax App](examples/federal-tax.md) and
  [Sprinkler weather control](examples/weather-sprinkler.md): complete Apps.

The [guides](guides/index.md), [concepts](concepts/index.md),
[examples](examples/index.md), and [reference](reference/index.md) remain
available; they are also the material the assistants read.

[Documentation home](index.md)
