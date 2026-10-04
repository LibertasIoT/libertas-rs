# Using task controls

A task can expose actions, current information, and live updates. The App
defines those capabilities; the client presents the appropriate controls for
the values and operations the App supplies.

## Actions and results

Open a page's actions with its **Actions** button, or with a named button the
App gives a group of actions. An action may run immediately or ask you to
enter values first. Read the operation's description and check the entered
values before submitting it. An action the App marks as critical first shows
a dialog naming it, with **Send** and **Cancel**; Cancel sends nothing.

While the App calculates values for the form, **Submit** is temporarily
disabled; the fields stay editable and links remain available. Once you send
a request, the page waits for its reply: other actions and links are
unavailable until it arrives.

The result may open a new page, update current information, or simply
acknowledge that the operation completed. An acknowledgement does not replace
the data you were viewing. An error keeps your entry so that you can correct
the problem and retry. Live updates can refresh the displayed information
independently of an action's reply.

## Find your way through a task

The App decides each next page; the client keeps no history of earlier pages.
The client's normal Back action therefore leaves the current page for the
task's starting page, or leaves the task, rather than stepping back through
earlier requests. Inside an interview-style workflow, Back leaves the
workflow; use the editor's own **Go back** action, where the App offers one,
to return to an earlier step. On a wide screen an App can show a list and an
editor side by side: the list on the left owns navigation, and the right side
offers **Close** instead of Back or Cancel.

A page may also offer **AI Search** to find one of its destinations from a
typed or spoken description; it only opens a page and never submits a form.

Opening a chart's full view gives it more room; the inline preview is a
starting point rather than a separate source of information. See
[Charts and visual results](visualization.md) for how to read these views.

Some operations require additional permissions. A missing action can also
reflect what the App actually provides, rather than a connection problem.
Ask an administrator to check access when needed, and use
[Troubleshooting](troubleshooting.md) for failed requests or missing results.

For exact native-client screen selection, Configure and Requests behavior,
Back/cancel outcomes, acknowledgements, and subscription lifetime, use the
[endpoint presentation reference](../reference/endpoint-presentation.md).
The [protocol reference](../reference/protocols.md) owns
[critical-request confirmation](../reference/protocols.md#request-confirmation),
[leaving while a reply is pending](../reference/protocols.md#exit-while-pending),
[landing and Back](../reference/protocols.md#control-endpoint-landing-and-native-back),
and [AI Search](../reference/protocols.md#ai-navigation-and-ai-search);
side-by-side panes follow the
[master-detail pane contract](../reference/presentation.md#master-detail-panes).

[All human guides](index.md)
