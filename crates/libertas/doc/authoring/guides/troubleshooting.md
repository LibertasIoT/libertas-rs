# Troubleshooting

Locate the stage that failed before changing the App. A source error, a
dependency problem, a Hub authorization failure, and a runtime error need
different corrections.

| Symptom | First useful check |
| --- | --- |
| Studio cannot access a Hub | Confirm the selected Hub, connection, and authorization. |
| Schema generation reports a file and line | Read the owning source declaration and correct it there. |
| Deployment stops at a package | Find that package's first actionable Build Monitor error. |
| A schema digest is stale | Regenerate from source with the current Studio version; do not patch the digest by hand. |
| Publication reports an invalid deployed schema or an undefined dependency version | Regenerate and redeploy the App and its local dependencies with current Studio. Older `minimumVersion` entries must become required `Version` pins through deployment; see [exact dependencies](../reference/deployment.md). |
| A function cannot be selected | Check installed version, task permissions, configuration access, and singleton restrictions. |
| A translation is not saved | Distinguish import validation from Hub upload success and retain the result file for retry. |
| A chart is unavailable | Inspect the ordinary data view and the client's supported chart behavior. |
| A running task stopped and reports an error | A failed data read or write stops the task unless its function opted into checked data errors. The Hub keeps the task's enabled setting but never restarts it automatically: correct the storage problem, then enable the task again. See the [Data API failure contract](../reference/protocols.md#persistence-failure-and-the-no-status-api). |

For Studio Plaza sign-in or publication failures, open **View → Output**, select
**Libertas Plaza**, and repeat the failed action. The log shows browser callback
completion, credential reuse or replacement, and request status codes. Include
the entries around the failure when reporting it.
The browser's authorization completion page confirms the return to Studio;
check Studio for the result of the authorization and publication.

Record the App/package name, operation, client version, Hub version, and a
bounded error message. Exclude credentials and private task data from anything
you share publicly. Screenshots should show the relevant state without
revealing unrelated account or device information.

Return to the relevant [human guide](index.md) for the normal workflow. The
[shared reference](../reference/index.md) defines exact schema, protocol,
versioning, and translation rules so that corrections address the cause.

[Documentation home](../index.md)
