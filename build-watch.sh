#!/bin/bash
# Watch mode - automatically rebuild on file changes
exec "$(dirname "$0")/build.sh" --watch
