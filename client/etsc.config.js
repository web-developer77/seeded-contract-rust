const esbuildPluginTsc = require("esbuild-plugin-tsc");

module.exports = {
  outDir: "./dist",
  esbuild: {
    platform: "node",
    target: "node14",
    entryPoints: ["src/index.ts"],
    outdir: "dist",
    format: "cjs",
    bundle: true,
    minify: true,
    sourcemap: true,
    plugins: [esbuildPluginTsc()],
  },
  assets: {
    baseDir: "src",
    outDir: "./dist",
    filePatterns: ["**/*.json"],
  },
};
