#!/usr/bin/env node

"use strict";

const path = require("path");
const fs = require("fs");
const { spawnSync } = require("child_process");
const os = require("os");

function getPlatform() {
  const arch = process.arch; // x64, arm64
  const platform = process.platform; // win32, darwin, linux

  if (platform === "win32" && arch === "x64") return "x86_64-pc-windows-msvc";
  if (platform === "win32" && arch === "arm64") return "aarch64-pc-windows-msvc";
  if (platform === "darwin" && arch === "x64") return "x86_64-apple-darwin";
  if (platform === "darwin" && arch === "arm64") return "aarch64-apple-darwin";
  if (platform === "linux" && arch === "x64") return "x86_64-unknown-linux-gnu";
  if (platform === "linux" && arch === "arm64") return "aarch64-unknown-linux-gnu";

  throw new Error(`Unsupported platform: ${platform}-${arch}`);
}

const platform = getPlatform();
const binDir = path.join(__dirname);
let exeName;

if (process.platform === "win32") {
  exeName = `idontnote-${platform}.exe`;
} else {
  exeName = `idontnote-${platform}`;
}

const exePath = path.join(binDir, exeName);

if (!fs.existsSync(exePath)) {
  console.error(`Error: Binary not found for your platform (${platform})`);
  console.error(`Expected: ${exePath}`);
  console.error("Please report this issue at: https://github.com/<your-username>/idont-notebook/issues");
  process.exit(1);
}

// Make executable on Unix
if (process.platform !== "win32") {
  try {
    fs.chmodSync(exePath, 0o755);
  } catch (_) {
    // Ignore permission errors
  }
}

const result = spawnSync(exePath, process.argv.slice(2), {
  stdio: "inherit",
});

process.exit(result.status ?? 1);
