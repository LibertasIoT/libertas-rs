'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { execFileSync, spawnSync } = require('node:child_process');
const { generate, sync, verifyCommit, main } = require('./sync-docs.cjs');
const git = (root, ...args) => execFileSync('git', ['-C', root, ...args], { encoding: 'utf8' }).trim();

test('SDK publishes only canonical links and checks the actual outgoing ref', () => {
  const sdk = fs.mkdtempSync(path.join(os.tmpdir(), 'libertas-sdk-links-'));
  try {
    git(sdk, 'init', '-q');
    git(sdk, 'config', 'user.name', 'Documentation test');
    git(sdk, 'config', 'user.email', 'docs@example.invalid');
    const expected = generate();
    assert.equal(expected.size, 3);
    assert.ok([...expected.values()].every(bytes => bytes.includes('https://github.com/LibertasIoT/libertas-doc/blob/main/')));
    sync(sdk, expected, false);
    sync(sdk, expected, true);
    git(sdk, 'add', '.'); git(sdk, 'commit', '-qm', 'Canonical link fixture');
    const published = git(sdk, 'rev-parse', 'HEAD');
    verifyCommit(sdk, published, expected);
    const entry = path.join(sdk, 'crates/libertas/CODEBOT.md');
    fs.appendFileSync(entry, 'stale');
    assert.throws(() => sync(sdk, expected, true), /stale/);
    git(sdk, 'add', '.'); git(sdk, 'commit', '-qm', 'Stale link fixture');
    const stale = git(sdk, 'rev-parse', 'HEAD');
    sync(sdk, expected, false);
    const prePush = sha => spawnSync(process.execPath,
      [path.join(__dirname, 'sync-docs.cjs'), '--sdk', sdk, '--pre-push'],
      { input: `refs/heads/main ${sha} refs/heads/main ${'0'.repeat(40)}\n`, encoding: 'utf8',
        env: { ...process.env, GIT_DIR: path.join(sdk, '.git'), GIT_WORK_TREE: sdk, GIT_INDEX_FILE: path.join(sdk, '.git/index'), LIBERTAS_SOURCE_ROOT: '/unavailable' } });
    assert.equal(prePush(published).status, 0);
    assert.equal(prePush(stale).status, 1);
    assert.equal(prePush('0'.repeat(40)).status, 0);
    verifyCommit(sdk, published, expected);
    const mirror = path.join(sdk, 'crates/libertas/doc/authoring');
    fs.mkdirSync(mirror, { recursive: true });
    fs.writeFileSync(path.join(mirror, 'index.md'), 'duplicate');
    assert.throws(() => sync(sdk, expected, false), /obsolete authoring mirror/);
    git(sdk, 'add', '.'); git(sdk, 'commit', '-qm', 'Mirror fixture');
    assert.throws(() => verifyCommit(sdk, 'HEAD', expected), /documentation mirror/);
    fs.rmSync(mirror, { recursive: true });
    fs.unlinkSync(entry);
    fs.symlinkSync('/missing/documentation-link-target', entry);
    assert.throws(() => sync(sdk, expected, false), /symlink/);
    fs.unlinkSync(entry);
    sync(sdk, expected, false);
    const hook = path.join(sdk, '.git/hooks/pre-push');
    fs.writeFileSync(hook, '# existing hook\n');
    assert.throws(() => main(['--sdk', sdk, '--install-hook']), /Existing pre-push hook/);
    assert.equal(fs.readFileSync(hook, 'utf8'), '# existing hook\n');
  } finally { fs.rmSync(sdk, { recursive: true, force: true }); }
});
