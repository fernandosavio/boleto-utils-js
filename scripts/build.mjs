import * as esbuild from 'esbuild'

const sharedConfig = {
  entryPoints: ['src/index.ts'],
  bundle: true,
  minify: true,
};

// For CJS
await esbuild.build({
  ...sharedConfig,
  platform: "node",
  target: ['node20'],
  outfile: "dist/index.js",
});

// For ESM
await esbuild.build({
  ...sharedConfig,
  platform: "neutral",
  outfile: "dist/index.esm.js",
  format: "esm",
});
