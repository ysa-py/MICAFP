#!/usr/bin/env node

const ACTIVE_STATUSES = new Set([
  'action_required',
  'in_progress',
  'pending',
  'queued',
  'requested',
  'waiting',
]);

const KEEP_LAST_N = Number.parseInt(process.env.KEEP_LAST_N ?? '25', 10);
const REPOSITORY = process.env.GITHUB_REPOSITORY;
const TOKEN = process.env.GITHUB_TOKEN;
const CURRENT_RUN_ID = process.env.GITHUB_RUN_ID;
const DRY_RUN = process.env.DRY_RUN === 'true';

if (!Number.isInteger(KEEP_LAST_N) || KEEP_LAST_N < 0) {
  throw new Error(`KEEP_LAST_N must be a non-negative integer, got ${process.env.KEEP_LAST_N}`);
}

if (!REPOSITORY) {
  throw new Error('GITHUB_REPOSITORY is required');
}

if (!TOKEN) {
  throw new Error('GITHUB_TOKEN is required');
}

const headers = {
  Accept: 'application/vnd.github+json',
  Authorization: `Bearer ${TOKEN}`,
  'X-GitHub-Api-Version': '2022-11-28',
};

async function githubJson(url) {
  const response = await fetch(url, { headers });
  if (!response.ok) {
    throw new Error(`GitHub API request failed: ${response.status} ${response.statusText} for ${url}`);
  }
  return response.json();
}

async function collectWorkflowRuns() {
  const allRuns = [];

  for (let page = 1; ; page += 1) {
    const url = new URL(`https://api.github.com/repos/${REPOSITORY}/actions/runs`);
    url.searchParams.set('per_page', '100');
    url.searchParams.set('page', String(page));

    const data = await githubJson(url);
    const runs = data.workflow_runs ?? [];
    allRuns.push(...runs);

    if (runs.length < 100) {
      break;
    }
  }

  return allRuns;
}

async function deleteWorkflowRun(run) {
  if (DRY_RUN) {
    console.log(`Dry run: would delete completed run ${run.id} (${run.status})`);
    return;
  }

  const response = await fetch(
    `https://api.github.com/repos/${REPOSITORY}/actions/runs/${run.id}`,
    { method: 'DELETE', headers },
  );

  if (!response.ok) {
    throw new Error(`Failed to delete run ${run.id}: ${response.status} ${response.statusText}`);
  }

  console.log(`Deleted completed run ${run.id} (${run.status})`);
}

async function main() {
  const allRuns = await collectWorkflowRuns();
  const completedRuns = allRuns.filter((run) => !ACTIVE_STATUSES.has(run.status));
  const protectedCompletedRuns = new Set(
    completedRuns.slice(0, KEEP_LAST_N).map((run) => run.id),
  );

  console.log(`Total runs: ${allRuns.length}`);
  console.log(`Active runs skipped: ${allRuns.length - completedRuns.length}`);
  console.log(`Protected completed runs: ${protectedCompletedRuns.size}`);

  for (const run of allRuns) {
    if (ACTIVE_STATUSES.has(run.status)) {
      console.log(`Skipping active run ${run.id} (${run.status})`);
      continue;
    }

    if (protectedCompletedRuns.has(run.id)) {
      console.log(`Protected completed runs: keeping run ${run.id} (${run.status})`);
      continue;
    }

    if (CURRENT_RUN_ID && String(run.id) === String(CURRENT_RUN_ID)) {
      console.log(`Skipping current run ${run.id} via independent self-protection guard`);
      continue;
    }

    await deleteWorkflowRun(run);
  }
}

await main();
