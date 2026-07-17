import type { ApplyAxiomPatchClientResult, PatchOp } from "../lsp/protocol";
import { applyAxiomPatch } from "../lsp/client";
import { patchFailureMessage } from "../lsp/patchFeedback";
import { isNotIndexedError } from "../utils/lspErrors";
import { workspaceEventBus } from "./eventBus";
import { ontologyRegistry } from "./ontologyRegistry";
import {
  decideCommitBookkeeping,
  shouldPopStackAfterApply,
} from "./transactionApplyPolicy";
import type { CommittedTransaction, PendingTransaction } from "./types";

const MAX_UNDO = 50;

type ApplyPatches = Array<PatchOp | ({ op: string } & Record<string, unknown>)>;

export class WorkspaceTransactionManager {
  private pending: PendingTransaction | undefined;
  private undoStack: CommittedTransaction[] = [];
  private redoStack: CommittedTransaction[] = [];

  async begin(
    documentUri: string,
    documentPath: string,
    patches: ApplyPatches,
    label?: string
  ): Promise<PendingTransaction> {
    if (this.pending) {
      throw new Error("A workspace transaction is already in progress");
    }
    // Reserve before await so concurrent begin() cannot overwrite (#386).
    this.pending = { documentUri, documentPath, patches, label };
    try {
      // Sync registry before assert so cold-start / post-NOT_INDEXED edits work (#301).
      try {
        await ontologyRegistry.syncFromCatalog();
      } catch (err) {
        if (!isNotIndexedError(err)) {
          throw err;
        }
      }
      ontologyRegistry.assertEditable(documentPath);
      return this.pending;
    } catch (err) {
      this.pending = undefined;
      throw err;
    }
  }

  async commit(): Promise<ApplyAxiomPatchClientResult> {
    const txn = this.pending;
    if (!txn) {
      throw new Error("No workspace transaction to commit");
    }
    try {
      const result = await applyAxiomPatch({
        document_uri: txn.documentUri,
        patches: txn.patches as ApplyPatches,
        preview_only: false,
      });
      const decision = decideCommitBookkeeping(result);
      if (decision.throwNotApplied) {
        throw new Error(patchFailureMessage(result));
      }
      // #297: mark dirty + stack undo even when editor sync was cancelled.
      if (decision.markDirty) {
        ontologyRegistry.markDirty(txn.documentPath);
      }
      if (decision.pushUndo) {
        this.undoStack.push({
          documentUri: txn.documentUri,
          documentPath: txn.documentPath,
          undoPatches: result.undo_patches ?? [],
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
    } finally {
      this.pending = undefined;
    }
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
    const txn = this.undoStack[this.undoStack.length - 1];
    if (!txn) {
      return false;
    }
    const result = await applyAxiomPatch({
      document_uri: txn.documentUri,
      patches: txn.undoPatches as ApplyPatches,
      preview_only: false,
    });
    if (!shouldPopStackAfterApply(result)) {
      throw new Error(patchFailureMessage(result));
    }
    // #296: pop only after successful apply.
    this.undoStack.pop();
    const decision = decideCommitBookkeeping(result);
    if (decision.markDirty) {
      ontologyRegistry.markDirty(txn.documentPath);
    }
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
    const txn = this.redoStack[this.redoStack.length - 1];
    if (!txn) {
      return false;
    }
    const result = await applyAxiomPatch({
      document_uri: txn.documentUri,
      patches: txn.undoPatches as ApplyPatches,
      preview_only: false,
    });
    if (!shouldPopStackAfterApply(result)) {
      throw new Error(patchFailureMessage(result));
    }
    this.redoStack.pop();
    const decision = decideCommitBookkeeping(result);
    if (decision.markDirty) {
      ontologyRegistry.markDirty(txn.documentPath);
    }
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
    patches: ApplyPatches,
    label?: string
  ): Promise<ApplyAxiomPatchClientResult> {
    await this.begin(documentUri, documentPath, patches, label);
    try {
      return await this.commit();
    } catch (err) {
      this.rollback();
      throw err;
    }
  }

  resetForTests(): void {
    this.pending = undefined;
    this.undoStack = [];
    this.redoStack = [];
  }
}

export const workspaceTransactionManager = new WorkspaceTransactionManager();
