import * as fs from "fs";
import * as path from "path";
import Mocha from "mocha";

function collectTestFiles(dir: string): string[] {
  const files: string[] = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...collectTestFiles(full));
    } else if (entry.name.endsWith(".test.js")) {
      files.push(full);
    }
  }
  return files;
}

export function run(): Promise<void> {
  const mocha = new Mocha({ ui: "tdd", timeout: 120_000 });
  const suiteRoot = __dirname;

  for (const file of collectTestFiles(suiteRoot)) {
    mocha.addFile(file);
  }

  return new Promise((resolve, reject) => {
    mocha.run((failures) => {
      if (failures > 0) {
        reject(new Error(`${failures} VS Code e2e test(s) failed`));
      } else {
        resolve();
      }
    });
  });
}
