/**
 * GitHub Actions workflow run cleanup helper.
 *
 * Intended for use from actions/github-script with `github`, `context`, and
 * `core` in scope. The sorting helper is exported for lightweight validation.
 */

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

async function cleanupWorkflowRuns({ github, context, core, keepRuns = 20, perPage = 100 } = {}) {
  if (!github || !context) {
    throw new Error("cleanupWorkflowRuns requires github and context objects");
  }

  const owner = context.repo.owner;
  const repo = context.repo.repo;
  const workflow_id = context.workflow;

  const runsRes = await github.rest.actions.listWorkflowRuns({
    owner,
    repo,
    workflow_id,
    per_page: perPage,
  });

  const sortedRuns = runsRes.data.workflow_runs
    ? [...runsRes.data.workflow_runs].sort(runCreatedAtThenNumberDesc)
    : [...runsRes.data].sort(runCreatedAtThenNumberDesc);

  const protectedRuns = sortedRuns.slice(0, keepRuns);
  const candidateRuns = sortedRuns.slice(keepRuns);
  const protectedIds = new Set(protectedRuns.map((run) => run.id));

  for (const run of candidateRuns) {
    if (protectedIds.has(run.id)) {
      continue;
    }

    await github.rest.actions.deleteWorkflowRun({
      owner,
      repo,
      run_id: run.id,
    });
    core?.info?.(`Deleted workflow run ${run.id} (#${run.run_number})`);
  }

  return { protectedRuns, candidateRuns };
}

module.exports = {
  cleanupWorkflowRuns,
  runCreatedAtThenNumberDesc,
};
