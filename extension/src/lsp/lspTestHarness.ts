import { spawn } from "node:child_process";
import * as fs from "node:fs";
import * as path from "node:path";

const REPO_ROOT = path.resolve(__dirname, "..", "..", "..");

export function resolveLspBinaryForTests(): string {
  const fromEnv = process.env.ONTOINDEX_LSP_BIN?.trim();
  if (fromEnv && fs.existsSync(fromEnv)) {
    return fromEnv;
  }

  const candidates = [
    path.join(REPO_ROOT, "target", "debug", "ontoindex-lsp"),
    path.join(REPO_ROOT, "target", "release", "ontoindex-lsp"),
  ];
  for (const candidate of candidates) {
    if (fs.existsSync(candidate)) {
      return candidate;
    }
  }

  throw new Error(
    "ontoindex-lsp binary not found; run `cargo build -p ontoindex-lsp --bins` or set ONTOINDEX_LSP_BIN"
  );
}

function writeLspMessage(stdin: NodeJS.WritableStream, body: string): void {
  const bytes = Buffer.byteLength(body, "utf8");
  stdin.write(`Content-Length: ${bytes}\r\n\r\n${body}`);
}

function readLspMessage(stdout: NodeJS.ReadableStream): Promise<string | null> {
  return new Promise((resolve, reject) => {
    let buffer = Buffer.alloc(0);

    const onData = (chunk: Buffer) => {
      buffer = Buffer.concat([buffer, chunk]);
      while (true) {
        const headerEnd = buffer.indexOf("\r\n\r\n");
        if (headerEnd === -1) {
          return;
        }

        const header = buffer.subarray(0, headerEnd).toString("utf8");
        const match = /Content-Length:\s*(\d+)/i.exec(header);
        if (!match) {
          reject(new Error(`missing Content-Length in LSP header: ${header}`));
          return;
        }

        const length = Number.parseInt(match[1]!, 10);
        const bodyStart = headerEnd + 4;
        if (buffer.length < bodyStart + length) {
          return;
        }

        const body = buffer.subarray(bodyStart, bodyStart + length).toString("utf8");
        stdout.off("data", onData);
        resolve(body);
        return;
      }
    };

    stdout.on("data", onData);
    stdout.on("error", reject);
    stdout.on("end", () => resolve(null));
  });
}

/** Minimal LSP handshake proving the binary can be spawned (not EACCES). */
export async function smokeInitializeLsp(binaryPath: string): Promise<void> {
  const child = spawn(binaryPath, [], {
    stdio: ["pipe", "pipe", "pipe"],
  });

  const spawnError = await new Promise<Error | null>((resolve) => {
    child.once("error", resolve);
    child.once("spawn", () => resolve(null));
  });
  if (spawnError) {
    throw spawnError;
  }

  const init = JSON.stringify({
    jsonrpc: "2.0",
    id: 1,
    method: "initialize",
    params: {
      processId: process.pid,
      rootUri: null,
      capabilities: {},
    },
  });
  writeLspMessage(child.stdin!, init);

  const response = await Promise.race([
    readLspMessage(child.stdout!),
    new Promise<null>((_, reject) =>
      setTimeout(() => reject(new Error("initialize response timeout")), 10_000)
    ),
  ]);

  child.kill();

  if (!response) {
    throw new Error("no initialize response from ontoindex-lsp");
  }
  const parsed = JSON.parse(response) as { result?: unknown; error?: unknown };
  if (parsed.error) {
    throw new Error(`initialize error: ${response}`);
  }
  if (!parsed.result) {
    throw new Error(`initialize missing result: ${response}`);
  }
}
