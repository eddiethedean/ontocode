import * as path from "path";
import * as vscode from "vscode";

/** Fixture workspace path (set by runVscodeTest / CI). */
export function fixturesWorkspaceUri(): string {
  const fromEnv = process.env.ONTOCODE_TEST_FIXTURES;
  if (fromEnv) {
    return vscode.Uri.file(fromEnv).toString();
  }
  return vscode.Uri.file(
    path.resolve(__dirname, "..", "..", "..", "..", "fixtures")
  ).toString();
}

/** Stable IRIs from fixtures/example.ttl */
export const FIXTURE_IRIS = {
  person: "http://example.org/people#Person",
  organization: "http://example.org/people#Organization",
  worksFor: "http://example.org/people#worksFor",
  alice: "http://example.org/people#alice",
} as const;
