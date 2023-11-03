import { spawn } from "node:child_process";
import { readFile } from "node:fs/promises";
import path from "node:path";

const { command, ports }: { command: string; ports: number[] } = JSON.parse(
  await readFile(path.resolve(process.cwd(), "scripts", "settings.json"), {
    encoding: "utf8",
  }),
);

const childProcess = ports.map((port) => {
  return spawn("cargo", ["watch", "-x", `run -- ${port}`], {
    stdio: "inherit",
  });
});

await new Promise((r) => {});
