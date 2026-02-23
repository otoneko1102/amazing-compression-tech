/**
 * postbuild: rewrite absolute asset paths to relative paths in docs/**\/*.html
 *
 * Astro without `base` outputs:  href="/_astro/index.abc.css"
 * This script rewrites to:       href="./_astro/index.abc.css"
 *
 * Result: the site works at any subpath (GitHub Pages, custom domain, etc.)
 * without hardcoding `base` in astro.config.mjs.
 */

import { readFileSync, writeFileSync, readdirSync, statSync } from 'fs';
import { resolve, relative, dirname } from 'path';
import { fileURLToPath } from 'url';

const docsDir = resolve(dirname(fileURLToPath(import.meta.url)), '../docs');

/** Recursively collect .html files under a directory */
function collectHtml(dir) {
  const results = [];
  for (const entry of readdirSync(dir)) {
    const full = resolve(dir, entry);
    if (statSync(full).isDirectory()) {
      results.push(...collectHtml(full));
    } else if (entry.endsWith('.html')) {
      results.push(full);
    }
  }
  return results;
}

const htmlFiles = collectHtml(docsDir);
let count = 0;

for (const file of htmlFiles) {
  const fileDir = dirname(file);
  let html = readFileSync(file, 'utf8');
  const before = html;

  // Rewrite every /_astro/... path in attribute values to a relative path
  html = html.replace(/="(\/_astro\/[^"]+)"/g, (_match, absPath) => {
    const absTarget = resolve(docsDir, absPath.slice(1)); // strip leading /
    const rel = relative(fileDir, absTarget).replace(/\\/g, '/');
    return `="${rel}"`;
  });

  if (html !== before) {
    writeFileSync(file, html, 'utf8');
    count++;
  }
}

console.log(`relative-paths: updated ${count} / ${htmlFiles.length} HTML file(s)`);
