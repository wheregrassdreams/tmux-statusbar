# tmux-statusbar

An independently installable TPM plugin for the tmux status line and native
command-prompt/message colors. It has no tmux-config Core dependency. The
session-name marquee uses prebuilt Rust release binaries. The plugin downloads
the matching binary on first load and verifies its SHA-256 checksum. Cargo and
a Rust toolchain are not required. If the release is unavailable, the platform
is unsupported, or the download cannot be verified, tmux itself truncates the
session name to the badge width.

## Install

```tmux
set -g @plugin 'wheregrassdreams/tmux-statusbar'
```

Install with TPM's `prefix + I`, then reload the tmux configuration.

## Colors and behavior

Colors are exposed as namespaced tmux options so users can set them before the
plugin is loaded. The defaults match this project's dark surface, muted text,
and lavender accent colors.

| Option | Default | Use |
| --- | --- | --- |
| `@tmux-statusbar-bg` | `default` | Status line background (terminal default/transparent) |
| `@tmux-statusbar-bg-session` | `#1B1B1B` | Inactive session badge background |
| `@tmux-statusbar-fg` | `#696969` | Inactive text and separators |
| `@tmux-statusbar-fg-current` | `#D8D8D8` | Current window text |
| `@tmux-statusbar-fg-session` | `#696969` | Session badge when prefix is inactive |
| `@tmux-statusbar-fg-prefix` | `#111111` | Prefix-active session badge text |
| `@tmux-statusbar-bg-prefix` | `#CBA6F7` | Prefix-active session badge background |
| `@tmux-statusbar-fg-accent` | `#CBA6F7` | Window indices and fullscreen icon |
| `@tmux-statusbar-fg-message` | `#D8D8D8` | Native prompt/message text and editable command-prompt input |
| `@tmux-statusbar-bg-message` | `#1B1B1B` | Native prompt/message background and fill |
| `@tmux-statusbar-bg-command` | `#1B1B1B` | Native command-prompt input background |
| `@tmux-statusbar-window-index-on-prefix` | `on` | Show window indices while prefix is active |
| `@tmux-statusbar-prefix-badge` | `on` | Highlight the session badge while prefix is active |
| `@tmux-statusbar-prefix-content` | `? keymaps` | Content shown at the right while prefix is active; accepts tmux formats such as `#(command)` |
| `@tmux-statusbar-session-marquee` | `on` | Scroll long session names in the left badge |
| `@tmux-statusbar-maximized-icon` | `󰊓` | Icon for the active zoomed window |

For example, override selected colors in `.tmux.conf` before TPM loads plugins:

```tmux
set -g @tmux-statusbar-bg '#111111'
set -g @tmux-statusbar-bg-prefix '#89B4FA'
set -g @tmux-statusbar-window-index-on-prefix off
set -g @tmux-statusbar-prefix-badge off
set -g @tmux-statusbar-prefix-content 'C-a'
set -g @tmux-statusbar-session-marquee off
```

The default prefix hint is `? keymaps`. Set `@tmux-statusbar-prefix-content` to
`C-a` to show the configured prefix instead, or use a tmux command format to
display script output:

```tmux
set -g @tmux-statusbar-prefix-content '#(~/bin/tmux-prefix-hint)'
```

The command runs according to tmux's status refresh interval. The status line
shows the hint only while prefix mode is active. Window indices and the session
badge highlight each have independent switches; both are enabled by default.
Native command prompts do not reset the client key table or force a status
redraw before opening. Their prompt area uses the same opaque gray background
and fill as status messages; command input uses the command background token
above. Prefix indicators may remain visible in the status content around the
prompt area until tmux redraws it.

The Rust renderer is cached under
`${XDG_CACHE_HOME:-~/.cache}/tmux-statusbar/<version>/<platform>`. It parses
grapheme clusters once, preallocates its segment list and output buffer, then
clears and reuses the same fixed-width frame buffer each second. A single stable
`#()` status job invokes it in auto mode: short names print once and exit,
while long names keep that job alive and stream one flushed line per frame.
When marquee is off, the same renderer prints a static, ellipsized label and
exits. Keeping the `#()` format present lets tmux notice name or option changes
and replace an old long-name worker when switching to a short name or turning
marquee off. In local measurements the resident Rust renderer used about
1.8 MiB RSS; short labels do not keep a renderer resident. The plugin downloads
only once per version and platform; if that is not possible, it uses tmux's
native truncation without starting a fallback renderer.

Set `@tmux-statusbar-session-marquee` to `off` to stop scrolling; long names
show a static prefix ending in a single `…` character.

The session badge uses a fixed 12-cell label, with tmux-native truncation for
long names. The status line itself is transparent; the inactive badge uses the
surface color and prefix mode fills it with the accent color. Window-list
formatting follows the original tmux-config layout: tmux centers the natural
window labels, shows indices in prefix mode, and places the zoom icon after the
current window label. Each item keeps the original one-cell trailing space so
the zoom icon does not push its separator away. No fixed-width slots are added
to the window list. A
muted `? keymaps` hint appears at the right in prefix mode; pressing `?` runs
tmux's native `list-keys` command. This does not depend on the optional
`tmux-keymaps` picker plugin. `message-style` and `message-command-style`
give native prompts and messages an opaque gray background and fill. Native
command-prompt labels keep tmux's own wording, with one cell of leading space
and the muted status foreground; editable input uses the message foreground.
