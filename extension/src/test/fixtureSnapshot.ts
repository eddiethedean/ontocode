import { CatalogSnapshot } from "../lsp/protocol";
import snapshot from "./fixture-catalog.snapshot.json";

export const fixtureCatalogSnapshot = snapshot as unknown as CatalogSnapshot;
