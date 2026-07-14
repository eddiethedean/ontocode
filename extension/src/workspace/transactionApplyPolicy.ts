/**
 * Pure policy for WorkspaceTransactionManager apply / undo / redo (#296, #297).
 */

export type PatchApplyLike = {
  applied: boolean;
  editor_synced: boolean;
  undo_patches?: unknown[] | null;
};

export type CommitBookkeeping = {
  /** Disk write succeeded — always mark dirty when applied. */
  markDirty: boolean;
  /** Record undo_patches when present (even if editor sync cancelled). */
  pushUndo: boolean;
  /** Caller should throw because nothing was applied. */
  throwNotApplied: boolean;
};

/** After a successful disk apply, always bookkeep; never drop undo for sync-cancel (#297). */
export function decideCommitBookkeeping(result: PatchApplyLike): CommitBookkeeping {
  if (!result.applied) {
    return { markDirty: false, pushUndo: false, throwNotApplied: true };
  }
  return {
    markDirty: true,
    pushUndo: (result.undo_patches?.length ?? 0) > 0,
    throwNotApplied: false,
  };
}

/**
 * Undo/redo stacks: peek then pop only after applied (#296).
 * Sync-cancel still counts as applied for stack mutation (#297).
 */
export function shouldPopStackAfterApply(result: PatchApplyLike): boolean {
  return result.applied;
}
