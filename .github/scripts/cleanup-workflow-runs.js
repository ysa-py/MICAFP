/**
 * GitHub Actions workflow run cleanup helper.
 *
 * Intended for use from actions/github-script with `github`, `context`, and
 * `core` in scope. The sorting helper is exported for lightweight validation.
 */

const ACTIVE_STATUSES = new Set([
  "in_progress",
  "pending",
  "queued",
  "requested",
  "waiting",
]);

function runCreatedAtThenNumberDesc(a, b) {
  const aTime = Date.parse(a?.created_at ?? "");
  const bTime = Date.parse(b?.created_at ?? "");
  const aTimeValid = Number.isFinite(aTime);
  const bTimeValid = Number.isFinite(bTime);

  if (aTimeValid && bTimeValid && aTime !== bTime) {
    return bTime - aTime;
  }

  return (b?.run_number ?? 0) - (a?.run_number ?? 0);
}

function runLabel(run) {
  return `run ${run?.id ?? "unknown"} (#${run?.run_number ?? "unknown"}, status: ${run?.status ?? "unknown"})`;
}

function warn(core, message) {
  if (core?.warning) {
    core.warning(message);
  } else {
    console.warn(message);
  }
}

async function cleanupWorkflowRuns({
  github,
  context,
  core,
  keepRuns = 20,
  perPage = 100,
  strict = false,
} = {}) {
  if (!github || !context) {
    throw new Error("cleanupWorkflowRuns requires github and context objects");
  }

  const owner = context.repo.owner;
  const repo = context.repo.repo;
  const workflow_id = context.workflow;
  const currentRunId = context.runId ?? process.env.GITHUB_RUN_ID;

  let runs;
  try {
    const runsRes = await github.rest.actions.listWorkflowRuns({
      owner,
      repo,
      workflow_id,
      per_page: perPage,
    });
    runs = runsRes.data.workflow_runs ?? runsRes.data ?? [];
  } catch (error) {
    const message = `Cleanup skipped: cannot list workflow runs for ${workflow_id} — ${error.message || String(error)}`;
    if (strict) {
      throw new Error(message, { cause: error });
    }
    warn(core, message);
    return { protectedRuns: [], candidateRuns: [], deletedRuns: [], skippedRuns: [], failedRuns: [], listFailed: true };
  }

  const sortedRuns = [...runs].sort(runCreatedAtThenNumberDesc);
  const protectedRuns = sortedRuns.slice(0, keepRuns);
  const candidateRuns = sortedRuns.slice(keepRuns);
  const protectedIds = new Set(protectedRuns.map((run) => run.id));
  const deletedRuns = [];
  const skippedRuns = [];
  const failedRuns = [];

  for (const run of candidateRuns) {
    if (protectedIds.has(run.id)) {
      skippedRuns.push(run);
      continue;
    }

    if (currentRunId && String(run.id) === String(currentRunId)) {
      core?.info?.(`Skipping current workflow ${runLabel(run)} via self-protection guard`);
      skippedRuns.push(run);
      continue;
    }

    if (ACTIVE_STATUSES.has(run.status)) {
      core?.info?.(`Skipping active workflow ${runLabel(run)}`);
      skippedRuns.push(run);
      continue;
    }

    try {
      await github.rest.actions.deleteWorkflowRun({
        owner,
        repo,
        run_id: run.id,
      });
      deletedRuns.push(run);
      core?.info?.(`Deleted workflow ${runLabel(run)}`);
    } catch (error) {
      const message = `Failed to delete workflow ${runLabel(run)} — ${error.message || String(error)}`;
      if (strict) {
        throw new Error(message, { cause: error });
      }
      failedRuns.push({ run, error });
      warn(core, message);
    }
  }

  return { protectedRuns, candidateRuns, deletedRuns, skippedRuns, failedRuns, listFailed: false };
}

module.exports = {
  ACTIVE_STATUSES,
  cleanupWorkflowRuns,
  runCreatedAtThenNumberDesc,
};
