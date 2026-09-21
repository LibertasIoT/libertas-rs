#!/usr/bin/env node
'use strict';
// Maintainer entry point; consumers can read the committed bundle without its source checkout.
const fs = require('node:fs');
const path = require('node:path');
const { execFileSync } = require('node:child_process');
const sdk = path.resolve(__dirname, '..');
let source = process.env.LIBERTAS_SOURCE_ROOT;
if (!source) {
  try { source = execFileSync('git', ['-C', sdk, 'config', '--get', 'libertas.docsSourceRoot'], { encoding: 'utf8' }).trim(); } catch {}
}
if (!source) source = path.resolve(sdk, '../../../libertas');
const generator = path.join(source, 'vscode/scripts/generate-rust-sdk-docs.cjs');
try {
  if (!fs.existsSync(generator)) throw new Error('Documentation source checkout is unavailable. Set LIBERTAS_SOURCE_ROOT to the canonical Libertas checkout.');
  require(generator).main(['--sdk', sdk, ...process.argv.slice(2)]);
} catch (error) { console.error(error.message); process.exitCode = 1; }
