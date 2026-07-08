#!/bin/sh

ACTION="$1"

if [ "$ACTION" = "ui-view" ]; then
  # Minimal contract: return HTML via `view_html`.
  echo '{"view_html":"<h3>Demo plugin view</h3><p>This HTML came from <code>demo_ui_view.sh</code>.</p>"}'
  exit 0
fi

echo '{"logs":"unknown action"}'
exit 0

