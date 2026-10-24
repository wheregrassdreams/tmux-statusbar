use std::env;
use std::io::{self, BufWriter, Write};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

struct Segment<'a> {
    text: &'a str,
    cells: usize,
}

struct Renderer<'a> {
    name: &'a str,
    segments: Vec<Segment<'a>>,
    width: usize,
    scrolls: bool,
    frame: String,
}

impl<'a> Renderer<'a> {
    fn new(name: &'a str, width: usize) -> Self {
        let width = width.max(1);
        let segments: Vec<_> = name
            .grapheme_indices(true)
            .map(|(_, text)| Segment {
                text,
                cells: UnicodeWidthStr::width(text),
            })
            .collect();
        let scrolls = segments.iter().map(|part| part.cells).sum::<usize>() > width;
        let frame_capacity = name.len().saturating_add(width);

        Self {
            name,
            segments,
            width,
            scrolls,
            // Allocate once; clear and reuse this frame buffer on each tick.
            frame: String::with_capacity(frame_capacity),
        }
    }

    fn render(&mut self, epoch_second: u64) -> &str {
        self.frame.clear();
        if !self.scrolls {
            self.frame.push_str(self.name);
            return &self.frame;
        }

        let segment_count = self.segments.len();
        let cycle_len = segment_count + 3;
        let offset = epoch_second as usize % cycle_len;
        let mut used = 0;
        for index in 0..(cycle_len * 2) {
            let cycle_index = (offset + index) % cycle_len;
            if cycle_index >= segment_count {
                if used == self.width {
                    break;
                }
                self.frame.push(' ');
                used += 1;
                continue;
            }

            let part = &self.segments[cycle_index];
            if used + part.cells > self.width {
                break;
            }
            self.frame.push_str(part.text);
            used += part.cells;
        }
        for _ in used..self.width {
            self.frame.push(' ');
        }
        &self.frame
    }

    fn render_static(&mut self) -> &str {
        self.frame.clear();
        if !self.scrolls {
            self.frame.push_str(self.name);
            return &self.frame;
        }

        let content_width = self.width.saturating_sub(1);
        let mut used = 0;
        for part in &self.segments {
            if used + part.cells > content_width {
                break;
            }
            self.frame.push_str(part.text);
            used += part.cells;
        }
        self.frame.push('…');
        &self.frame
    }
}

fn safe_name(input: &str) -> String {
    let clean: String = input
        .chars()
        .map(|character| {
            if character.is_control() {
                ' '
            } else {
                character
            }
        })
        .collect();
    clean.replace("#[", "# [")
}

fn epoch_second() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn usage() -> ! {
    eprintln!("usage: session-marquee [--auto|--watch|--static] NAME WIDTH");
    std::process::exit(2);
}

fn main() {
    let mut args = env::args().skip(1);
    let first = args.next().unwrap_or_else(|| usage());
    let auto = first == "--auto";
    let watch = first == "--watch";
    let static_only = first == "--static";
    let name = if auto || watch || static_only {
        args.next().unwrap_or_else(|| usage())
    } else {
        first
    };
    let width = args
        .next()
        .unwrap_or_else(|| usage())
        .parse::<usize>()
        .unwrap_or_else(|_| {
            eprintln!("session-marquee: WIDTH must be an integer");
            std::process::exit(2);
        });
    if args.next().is_some() {
        usage();
    }

    let safe_name = safe_name(&name);
    let mut renderer = Renderer::new(&safe_name, width);
    // In auto mode, emit short labels once and exit. Long labels keep the
    // tmux format job alive and stream one frame per status interval.
    let watch = !static_only && (watch || (auto && renderer.scrolls));
    let stdout = io::stdout();
    let mut output = BufWriter::with_capacity(64, stdout.lock());
    let mut second = epoch_second();

    loop {
        let frame = if static_only {
            renderer.render_static()
        } else {
            renderer.render(second)
        };
        if output.write_all(frame.as_bytes()).is_err() {
            return;
        }
        if !watch {
            return;
        }
        if output
            .write_all(b"\n")
            .and_then(|_| output.flush())
            .is_err()
        {
            return;
        }
        thread::sleep(Duration::from_secs(1));
        second = second.wrapping_add(1);
    }
}
