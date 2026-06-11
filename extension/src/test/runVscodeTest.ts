import * as path from "path";
import {
  downloadAndUnzipVSCode,
  resolveCliPathFromVSCodeExecutablePath,
  runTests,
} from "@vscode/test-electron";

async function main(): Promise<void> {
  const extensionRoot = path.resolve(__dirname, "..", "..");
  const extensionTestsPath = path.resolve(__dirname, "suite", "index");
  const fixturesPath = path.resolve(extensionRoot, "..", "fixtures");
  const version = process.env.VSCODE_VERSION ?? "stable";

  const vscodeExecutablePath = await downloadAndUnzipVSCode(version);
  const vscodeCliPath = resolveCliPathFromVSCodeExecutablePath(
    vscodeExecutablePath
  );

  await runTests({
    vscodeExecutablePath: vscodeCliPath,
    extensionDevelopmentPath: extensionRoot,
    extensionTestsPath,
    extensionTestsEnv: {
      ONTOCODE_TEST_FIXTURES: fixturesPath,
    },
  });
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
