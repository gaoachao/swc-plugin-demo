import { execSync } from "node:child_process";
import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const pluginDir = path.join(__dirname, "my-first-plugin");

execSync(`cd ${pluginDir} && cargo build-wasi --release`, {
  env: {
    ...process.env,
    RUSTFLAGS: "-C link-arg=--export-table -C link-arg=-s",
  },
  stdio: "inherit",
});

await fs.copyFile(
  path.resolve(pluginDir, "target/wasm32-wasi/release/my_first_plugin.wasm"),
  path.resolve(__dirname, "my_first_plugin.wasm")
);
