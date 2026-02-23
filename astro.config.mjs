// @ts-check
import { defineConfig } from 'astro/config';

// https://astro.build/config
export default defineConfig({
  output: 'static', // GitHub Pages needs static output
  // Use docs as build directory for GitHub Pages
  outDir: 'docs',
  // Set base depending on repository. If deploying to a project page, uncomment and set:
  // base: '/amazing-compression-tech/',
  // site: 'https://yourusername.github.io/amazing-compression-tech',
});
