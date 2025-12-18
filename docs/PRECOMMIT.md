# Pre-commit integration (snippet)

Example `.git/hooks/pre-commit` (bash):

```bash
#!/usr/bin/env bash
set -euo pipefail

# run staged scope by default
veto scan --scope staged
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```
