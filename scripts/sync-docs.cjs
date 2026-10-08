#!/usr/bin/env node
'use strict';
const fs = require('node:fs');
const path = require('node:path');
const { execFileSync } = require('node:child_process');
const repository = 'https://github.com/LibertasIoT/libertas-doc';
const bundle = 'crates/libertas/doc/authoring';
const gitEnv = { ...process.env };
for (const name of execFileSync('git', ['rev-parse', '--local-env-vars'], { encoding: 'utf8' }).trim().split('\n')) delete gitEnv[name];
const git = (cwd, args, input) => execFileSync('git', ['-C', cwd, ...args], { input, encoding: 'utf8', env: gitEnv }).trim();

function generate() {
  return new Map(['CODEBOT', 'DEVELOPER-ASSISTANT', 'USER-ASSISTANT'].map(name => [
    `crates/libertas/${name}.md`,
    Buffer.from(`# Libertas documentation\n\nRead [${name}.md](${repository}/blob/main/${name}.md) in the canonical public\ndocumentation repository. Clone [libertas-doc](${repository}) for offline reading\nor choose a specific documentation commit. This SDK contains runtime APIs and\nlinks to the documentation; it does not mirror the authoring guides.\n`)
  ]));
}

function sync(sdk, expected, check) {
  if (fs.existsSync(path.join(sdk, bundle))) throw new Error('The obsolete authoring mirror must be removed; public documentation belongs only in libertas-doc.');
  let stale = 0;
  for (const [name, bytes] of expected) {
    const file = path.join(sdk, name);
    for (let current = file; current !== sdk; current = path.dirname(current)) {
      try {
        if (fs.lstatSync(current).isSymbolicLink()) throw new Error(`Refusing documentation link symlink: ${current}`);
      } catch (error) { if (error.code !== 'ENOENT') throw error; }
    }
    if (fs.existsSync(file) && fs.readFileSync(file).equals(bytes)) continue;
    stale++;
    if (!check) {
      fs.mkdirSync(path.dirname(file), { recursive: true });
      const temp = `${file}.${process.pid}.tmp`;
      try { fs.writeFileSync(temp, bytes); fs.renameSync(temp, file); }
      finally { fs.rmSync(temp, { force: true }); }
    }
  }
  if (check && stale) throw new Error('SDK documentation links are stale. Run scripts/sync-docs.cjs, review, and commit.');
  console.log(`${check ? 'Verified' : 'Updated'} three canonical documentation links; no authoring mirror.`);
}

function verifyCommit(sdk, revision, expected) {
  const tree = git(sdk, ['ls-tree', '-r', '-z', `${revision}^{tree}`, '--', bundle, ...expected.keys()]);
  const actual = new Map(tree.split('\0').filter(Boolean).map(line => {
    const match = /^(\d+) blob ([0-9a-f]+)\t([\s\S]+)$/.exec(line);
    if (!match || match[1] !== '100644') throw new Error(`Unexpected documentation entry in ${revision}`);
    return [match[3], match[2]];
  }));
  for (const [name, bytes] of expected) {
    if (actual.get(name) !== git(sdk, ['hash-object', '--stdin'], bytes)) throw new Error(`Push blocked: ${revision} has stale ${name}. Commit the canonical documentation links.`);
    actual.delete(name);
  }
  if (actual.size) throw new Error(`Push blocked: ${revision} still contains a public documentation mirror.`);
}

function installHook(sdk) {
  const hook = path.resolve(sdk, git(sdk, ['rev-parse', '--git-path', 'hooks/pre-push']));
  const content = '#!/bin/sh\n# libertas-sdk-docs managed hook\nexec node "$(git rev-parse --show-toplevel)/scripts/sync-docs.cjs" --pre-push "$@"\n';
  if (fs.existsSync(hook) && fs.readFileSync(hook, 'utf8') !== content) throw new Error(`Existing pre-push hook was left unchanged: ${hook}. Integrate scripts/sync-docs.cjs --pre-push into it.`);
  fs.mkdirSync(path.dirname(hook), { recursive: true });
  fs.writeFileSync(hook, content, { mode: 0o755 });
}

function main(args) {
  let sdk = path.resolve(__dirname, '..'), check = false, install = false, revision, prePush = false;
  for (let i = 0; i < args.length; i++) {
    if (args[i] === '--sdk' && args[i + 1]) sdk = path.resolve(args[++i]);
    else if (args[i] === '--check') check = true;
    else if (args[i] === '--install-hook') install = true;
    else if (args[i] === '--verify-commit' && args[i + 1]) revision = args[++i];
    else if (args[i] === '--pre-push') { prePush = true; break; }
    else throw new Error('Usage: sync-docs.cjs [--sdk PATH] [--check] [--install-hook] [--verify-commit REF]');
  }
  sdk = fs.realpathSync(sdk);
  const refs = prePush ? fs.readFileSync(0, 'utf8').trim().split('\n').filter(Boolean)
    .map(line => line.trim().split(/\s+/)[1]).filter(sha => !/^0+$/.test(sha)) : [];
  if (prePush && !refs.length) return;
  const expected = generate();
  sync(sdk, expected, check || prePush);
  if (revision) verifyCommit(sdk, revision, expected);
  for (const sha of new Set(refs)) verifyCommit(sdk, sha, expected);
  if (install) installHook(sdk);
}

if (require.main === module) {
  try { main(process.argv.slice(2)); }
  catch (error) { console.error(error.message); process.exitCode = 1; }
}
module.exports = { generate, sync, verifyCommit, main };
