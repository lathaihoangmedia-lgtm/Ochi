#!/usr/bin/env bash
set -euo pipefail

# Migrate legacy home directory (~/.openfang) to Ochi home (~/.ochi)
# with safe merge, timestamped backup, and rollback support.

usage() {
  cat <<'USAGE'
Usage:
  migrate-home.sh [--source PATH] [--target PATH] [--dry-run] [--yes] [--rollback BACKUP_PATH]

Options:
  --source PATH      Source home directory (default: ~/.openfang)
  --target PATH      Target home directory (default: ~/.ochi)
  --dry-run          Show planned actions without modifying files
  --yes              Skip confirmation prompt
  --rollback PATH    Restore from a backup directory created by this script
  -h, --help         Show this help

Behavior:
- Creates target directory if missing.
- Copies source into target WITHOUT overwriting existing files.
- Creates a timestamped backup directory containing:
  - source snapshot
  - target snapshot (before merge)
- Writes a migration report file.
USAGE
}

SOURCE_DEFAULT="${HOME}/.openfang"
TARGET_DEFAULT="${HOME}/.ochi"
source_dir="$SOURCE_DEFAULT"
target_dir="$TARGET_DEFAULT"
dry_run=0
auto_yes=0
rollback_dir=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --source)
      source_dir="$2"; shift 2 ;;
    --target)
      target_dir="$2"; shift 2 ;;
    --dry-run)
      dry_run=1; shift ;;
    --yes)
      auto_yes=1; shift ;;
    --rollback)
      rollback_dir="$2"; shift 2 ;;
    -h|--help)
      usage; exit 0 ;;
    *)
      echo "Unknown option: $1" >&2
      usage
      exit 1 ;;
  esac
done

if [[ -n "$rollback_dir" ]]; then
  if [[ ! -d "$rollback_dir" ]]; then
    echo "Rollback path does not exist: $rollback_dir" >&2
    exit 1
  fi

  target_snapshot="$rollback_dir/target_before"
  if [[ ! -d "$target_snapshot" ]]; then
    echo "Rollback directory missing target_before snapshot: $target_snapshot" >&2
    exit 1
  fi

  echo "[rollback] restoring target from: $target_snapshot"
  if [[ $dry_run -eq 1 ]]; then
    echo "[dry-run] rm -rf '$target_dir'"
    echo "[dry-run] cp -a '$target_snapshot' '$target_dir'"
    exit 0
  fi

  rm -rf "$target_dir"
  cp -a "$target_snapshot" "$target_dir"
  echo "[rollback] done"
  exit 0
fi

if [[ "$source_dir" == "$target_dir" ]]; then
  echo "Source and target must be different." >&2
  exit 1
fi

if [[ ! -d "$source_dir" ]]; then
  echo "Source directory not found: $source_dir"
  echo "Nothing to migrate."
  exit 0
fi

backup_root="${target_dir}.migration-backups"
timestamp="$(date +%Y%m%d-%H%M%S)"
backup_dir="$backup_root/$timestamp"

report_line() {
  printf '%s\n' "$1" >> "$backup_dir/migration-report.txt"
}

confirm() {
  if [[ $auto_yes -eq 1 ]]; then
    return 0
  fi
  read -r -p "Proceed migration from '$source_dir' to '$target_dir'? [y/N] " ans
  [[ "$ans" =~ ^[Yy]$ ]]
}

echo "[plan] source: $source_dir"
echo "[plan] target: $target_dir"
echo "[plan] backup: $backup_dir"

if ! confirm; then
  echo "Cancelled."
  exit 1
fi

if [[ $dry_run -eq 1 ]]; then
  echo "[dry-run] mkdir -p '$backup_dir'"
  echo "[dry-run] snapshot source and target"
  echo "[dry-run] mkdir -p '$target_dir'"
  echo "[dry-run] rsync -a --ignore-existing '$source_dir/' '$target_dir/'"
  echo "[dry-run] write migration report"
  exit 0
fi

mkdir -p "$backup_dir"

# Snapshot source and existing target for rollback.
cp -a "$source_dir" "$backup_dir/source_before"
if [[ -d "$target_dir" ]]; then
  cp -a "$target_dir" "$backup_dir/target_before"
else
  mkdir -p "$backup_dir/target_before"
fi

mkdir -p "$target_dir"

# Merge without overwrite.
if command -v rsync >/dev/null 2>&1; then
  rsync -a --ignore-existing "$source_dir/" "$target_dir/"
else
  # portable fallback: copy only missing files
  while IFS= read -r -d '' file; do
    rel="${file#${source_dir}/}"
    dest="$target_dir/$rel"
    if [[ ! -e "$dest" ]]; then
      mkdir -p "$(dirname "$dest")"
      cp -a "$file" "$dest"
    fi
  done < <(find "$source_dir" -mindepth 1 -print0)
fi

# Build report
report_line "Ochi home migration report"
report_line "timestamp: $timestamp"
report_line "source: $source_dir"
report_line "target: $target_dir"
report_line "backup: $backup_dir"

copied_count=$(python - <<PY
import os
s = {os.path.relpath(os.path.join(dp,f), '$source_dir') for dp,_,fs in os.walk('$source_dir') for f in fs}
t = {os.path.relpath(os.path.join(dp,f), '$target_dir') for dp,_,fs in os.walk('$target_dir') for f in fs}
print(len(s & t))
PY
)
report_line "files_present_in_target_after_merge: $copied_count"

report_line "rollback_command: scripts/migrate-home.sh --rollback '$backup_dir' --target '$target_dir'"

echo "Migration completed."
echo "Backup: $backup_dir"
echo "Report: $backup_dir/migration-report.txt"
