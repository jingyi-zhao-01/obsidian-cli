# Machine Contracts

> Available in v1.1.0+

For agent integration, use JSON output with deterministic contracts.

## JSON Output

```bash
# Human output (default)
obsidian-cli-inspector query search rust

# JSON output
obsidian-cli-inspector --json query search rust
obsidian-cli-inspector -o json query search rust
```

## JSON Response Schema

All commands return a consistent JSON structure:

```json
{
  "version": "1.0",
  "command": "query.search",
  "timestamp": "2026-02-28T20:26:00Z",
  "params": {
    "query": "rust",
    "limit": 20
  },
  "result": {
    "total": 5,
    "items": [...]
  },
  "meta": {
    "query_time_ms": 12,
    "vault_path": "/path/to/vault"
  }
}
```

## Stable Note Identifiers

Notes have a persistent UUID (`stable_id`) that survives renames:

```json
{
  "notes": [
    {
      "stable_id": "550e8400-e29b-41d4-a716-446655440000",
      "path": "folder/note.md",
      "title": "Note Title"
    }
  ]
}
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Database error |
| 4 | Vault not found |
| 5 | Not initialized |

In JSON mode, errors return structured response:
```json
{
  "error": {
    "code": 2,
    "message": "Invalid arguments: --limit must be positive"
  }
}
```

## Deterministic Sorting

All lists are sorted by primary field, then by `stable_id` as tie-breaker.
