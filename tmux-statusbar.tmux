#!/usr/bin/env bash
# Standalone tmux status bar plugin. No tmux-config Core protocol required.
set -eu

CURRENT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
configured_marquee="$(tmux show-option -gqv @tmux-statusbar-session-marquee 2>/dev/null || true)"
MARQUEE_MODE=truncate
MARQUEE_BINARY=""

# Install the matching prebuilt release into the user cache. A missing release,
# unsupported platform, or offline first load simply uses tmux's native format.
if [[ "$configured_marquee" != off ]] && MARQUEE_BINARY="$("$CURRENT_DIR/scripts/install-session-marquee" 2>/dev/null)"; then
  MARQUEE_MODE=rust
fi

set_default() {
  local option=$1 value=$2 current
  current=$(tmux show-option -gqv "$option" 2>/dev/null || true)
  [[ -n $current ]] || tmux set-option -g "$option" "$value"
}

set_default @tmux-statusbar-bg default
set_default @tmux-statusbar-bg-session '#1B1B1B'
set_default @tmux-statusbar-fg '#696969'
set_default @tmux-statusbar-fg-current '#D8D8D8'
set_default @tmux-statusbar-fg-session '#696969'
set_default @tmux-statusbar-fg-prefix '#111111'
set_default @tmux-statusbar-bg-prefix '#CBA6F7'
set_default @tmux-statusbar-fg-accent '#CBA6F7'
set_default @tmux-statusbar-fg-message '#D8D8D8'
set_default @tmux-statusbar-bg-message '#1B1B1B'
set_default @tmux-statusbar-bg-command '#1B1B1B'
set_default @tmux-statusbar-window-index-on-prefix on
set_default @tmux-statusbar-prefix-badge on
set_default @tmux-statusbar-prefix-content '? keymaps'
set_default @tmux-statusbar-session-marquee on
tmux set-option -g @_tmux-statusbar-session-marquee-mode "$MARQUEE_MODE"
if [[ "$MARQUEE_MODE" == rust ]]; then
  tmux set-option -g @_tmux-statusbar-session-marquee-binary "$MARQUEE_BINARY"
else
  tmux set-option -gu @_tmux-statusbar-session-marquee-binary 2>/dev/null || true
fi
tmux set-option -gu @_tmux-statusbar-session-marquee-script 2>/dev/null || true
tmux set-option -gu @_tmux-statusbar-session-label 2>/dev/null || true
set_default @tmux-statusbar-maximized-icon '󰊓'

# Retire the old cross-plugin hint slots; the native '?' hint is owned here.
tmux set-option -gu @tmux-statusbar-right-hint-prefix 2>/dev/null || true
tmux set-option -gu @tmux-statusbar-right-hint-key 2>/dev/null || true
tmux set-option -gu @tmux-statusbar-right-hint-label 2>/dev/null || true

tmux source-file "$CURRENT_DIR/statusbar.conf"
