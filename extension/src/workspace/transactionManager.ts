import type { ApplyAxiomPatchClientResult, PatchOp } from "../lsp/protocol";
import { applyAxiomPatch } from "../lsp/client";
import { requirePatchFullySynced } from "../lsp/patchFeedback";
import { isNotIndexedError } from "../utils/lspErrors";
import { workspaceEventBus } from "./eventBus";
import { ontologyRegistry } from "./ontologyRegistry";
import type { CommittedTransaction, PendingTransaction } from "./types";

const MAX_UNDO = 50;

export class WorkspaceTransactionManager {
  private pending: PendingTransaction | undefined;
  private undoStack: CommittedTransaction[] = [];
  private redoStack: CommittedTransaction[] = [];

  async begin(
    documentUri: string,
    documentPath: string,
    patches: PatchOp[],
    label?: string
  ): Promise<PendingTransaction> {
    if (this.pending) {
      throw new Error("A workspace transaction is already in progress");
    }
    // Sync registry before assert so cold-start / post-NOT_INDEXED edits work (#301).
    try {
      await ontologyRegistry.syncFromCatalog();
    } catch (err) {
      if (!isNotIndexedError(err)) {
        throw err;
      }
    }
    ontologyRegistry.assertEditable(documentPath);
    this.pending = { documentUri, documentPath, patches, label };
    return this.pending;
  }

  async commit(): Promise<ApplyAxiomPatchClientResult> {
    const txn = this.pending;
    if (!txn) {
      throw new Error("No workspace transaction to commit");
    }
    this.pending = undefined;
    const result = await applyAxiomPatch({
      document_uri: txn.documentUri,
      patches: txn.patches as PatchOp[],
      preview_only: false,
    });
    requirePatchFullySynced(result);
    ontologyRegistry.markDirty(txn.documentPath);
    const undoPatches = result.undo_patches ?? [];
    if (undoPatches.length > 0) {
      this.undoStack.push({
        documentUri: txn.documentUri,
        documentPath: txn.documentPath,
        undoPatches,
        label: txn.label,
      });
      this.redoStack = [];
      if (this.undoStack.length > MAX_UNDO) {
        this.undoStack.shift();
      }
    }
    workspaceEventBus.publish("TransactionCommitted", {
      documentPath: txn.documentPath,
      label: txn.label,
    });
    return result;
  }

  rollback(): void {
    this.pending = undefined;
  }

  canUndo(): boolean {
    return this.undoStack.length > 0;
  }

  canRedo(): boolean {
    return this.redoStack.length > 0;
  }

  async undo(): Promise<boolean> {
    const txn = this.undoStack.pop();
    if (!txn) {
      return false;
    }
    const result = await applyAxiomPatch({
      document_uri: txn.documentUri,
      patches: txn.undoPatches as PatchOp[],
      preview_only: false,
    });
    requirePatchFullySynced(result);
    ontologyRegistry.markDirty(txn.documentPath);
    if (result.undo_patches && result.undo_patches.length > 0) {
      this.redoStack.push({
        documentUri: txn.documentUri,
        documentPath: txn.documentPath,
        undoPatches: result.undo_patches,
        label: txn.label,
      });
    }
    workspaceEventBus.publish("TransactionUndone", {
      documentPath: txn.documentPath,
      label: txn.label,
    });
    return true;
  }

  async redo(): Promise<boolean> {
    const txn = this.redoStack.pop();
    if (!txn) {
      return false;
    }
    const result = await applyAxiomPatch({
      document_uri: txn.documentUri,
      patches: txn.undoPatches as PatchOp[],
      preview_only: false,
    });
    requirePatchFullySynced(result);
    ontologyRegistry.markDirty(txn.documentPath);
    if (result.undo_patches && result.undo_patches.length > 0) {
      this.undoStack.push({
        documentUri: txn.documentUri,
        documentPath: txn.documentPath,
        undoPatches: result.undo_patches,
        label: txn.label,
      });
    }
    workspaceEventBus.publish("TransactionRedone", {
      documentPath: txn.documentPath,
      label: txn.label,
    });
    return true;
  }

  async apply(
    documentUri: string,
    documentPath: string,
    patches: PatchOp[],
    label?: string
  ): Promise<ApplyAxiomPatchClientResult> {
    await this.begin(documentUri, documentPath, patches, label);
    return this.commit();
  }

  resetForTests(): void {
    this.pending = undefined;
    this.undoStack = [];
    this.redoStack = [];
  }
}

export const workspaceTransactionManager = new WorkspaceTransactionManager();
