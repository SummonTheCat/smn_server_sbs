#!/usr/bin/env bash
set -euo pipefail

# Ensure we're in a git repo
if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "Error: not inside a git repository"
  exit 1
fi

# Determine current branch
BRANCH="$(git rev-parse --abbrev-ref HEAD)"

echo "Resetting repository to origin/${BRANCH}"
echo "This will DELETE all local changes and untracked files."
echo

# Fetch latest state from remote
git fetch --all --prune

# Hard reset tracked files to remote
git reset --hard "origin/${BRANCH}"

# Remove all untracked files and directories (including ignored ones)
git clean -fdx

echo
echo "Repository successfully synced to origin/${BRANCH}"
