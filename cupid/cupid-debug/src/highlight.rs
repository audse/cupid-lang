use crate::severity::Severity;
use colored::*;
use cupid_lex::{span::Position, token::Token};
use cupid_util::{
    fmt::{draw_line, draw_underline},
    lines,
};
use std::fmt;

const PEEK: usize = 2;

/// # Examples
/// ```
/// LineSet {
///     overline:  String::from("    │                "),
///     line:      String::from(" 12 │ let my_var = 1 "),
///     underline: String::from("    │     ^^^^^^     "),
/// }
/// ```
#[derive(Default)]
pub struct LineSet {
    pub overline: String,
    pub line: String,
    pub underline: String,
}

impl fmt::Display for LineSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}", self.overline, self.line, self.underline)
    }
}

fn token_line_range(token: &Token<'static>) -> (usize, usize) {
    let range = || token.span.start.line..=token.span.end.line;
    (
        range().min().unwrap_or_default(),
        range().max().unwrap_or_default(),
    )
}

#[derive(Default)]
pub struct HighlightedLineSet {
    pub position: Position,
    pub lines: Vec<HighlightedLine>,
    pub context: (Vec<HighlightedLine>, Vec<HighlightedLine>),
}

impl HighlightedLineSet {
    pub fn new(tokens: &[&Token<'static>], severity: Severity, source: &str) -> Self {
        let source_lines = lines!(source);
        let start_position = tokens
            .iter()
            .min_by(|x, y| x.span.cmp(&y.span))
            .map(|t| t.span.start)
            .unwrap_or_default();
        let end_position = tokens
            .iter()
            .max_by(|x, y| x.span.cmp(&y.span))
            .map(|t| t.span.end)
            .unwrap_or_default();
        let mut lines = vec![];
        for token in tokens {
            let (first, last) = token_line_range(token);

            for line_num in token.span.start.line..=token.span.end.line {
                let mut range = (token.span.start.character, token.span.end.character);
                if line_num != first {
                    range.0 = 0;
                }
                if line_num != last {
                    range.1 = source_lines[line_num].len();
                }
                lines.push(HighlightedLine::new(
                    source_lines[line_num],
                    line_num,
                    range,
                    severity,
                ));
            }
        }

        let context_line = |num: usize| -> HighlightedLine {
            HighlightedLine::new(source_lines[num], num, (0, 0), severity)
        };

        let mut context = (vec![], vec![]);
        for peek in 1..(PEEK + 1) {
            if start_position.line > peek {
                context
                    .0
                    .insert(0, context_line(start_position.line - peek))
            }
            if source_lines.len() > end_position.line + peek {
                context.1.push(context_line(end_position.line + peek))
            }
        }
        Self {
            position: start_position,
            context,
            lines,
        }
    }
    pub fn finish(self) -> String {
        let line_info = format!(
            "      ╭─ main.cupid ── line {}, char {}",
            self.position.line, self.position.character
        )
        .dimmed()
        .italic();
        let inner = self
            .lines
            .into_iter()
            .map(|line| line.finish())
            .collect::<Vec<String>>()
            .join("\n");
        let fmt_context = |context: Vec<HighlightedLine>, above: bool| {
            let len = context.len();
            context
                .into_iter()
                .enumerate()
                .map(|(i, line)| {
                    if above && i == 0 {
                        let lines = line.indent().lines;
                        format!("{}\n{}\n", lines.overline, lines.line)
                    } else if !above && i == len - 1 {
                        let lines = line.indent().lines;
                        format!("{}\n{}\n", lines.line, lines.underline)
                    } else {
                        format!("{}\n", line.indent().lines.line)
                    }
                    .dimmed()
                    .to_string()
                })
                .collect::<Vec<String>>()
                .join("")
        };
        let context_above = fmt_context(self.context.0, true);
        let context_below = fmt_context(self.context.1, false);
        let corner = format!("      ╰───╼").dimmed();
        format!("{line_info}\n{context_above}{inner}\n{context_below}{corner}")
    }
}

#[derive(Default)]
pub struct HighlightedLine {
    pub lines: LineSet,
    pub line_num: usize,
    pub range: (usize, usize),
    pub severity: Severity,
}

impl HighlightedLine {
    pub fn new(line: &str, line_num: usize, range: (usize, usize), severity: Severity) -> Self {
        Self {
            lines: LineSet {
                line: line.to_string(),
                ..Default::default()
            },
            line_num,
            range,
            severity,
        }
    }
    pub fn bold_range(mut self) -> Self {
        self.lines.line = self
            .lines
            .line
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i >= self.range.0 && i <= self.range.1 {
                    format!("{}", c.to_string().bold())
                } else {
                    c.to_string()
                }
            })
            .collect();
        self
    }
    pub fn underline_range(mut self) -> Self {
        let underline = draw_underline(if self.range.1 > self.range.0 {
            self.range.1 - self.range.0
        } else {
            0
        }); // e.g. "^^^^"

        let before = draw_line(
            " ",
            if self.range.0 > 0 {
                self.range.0 - 1
            } else {
                0
            },
        ); // e.g. "  "
        let underline = format!("{before}{underline}"); // e.g. "  ^^^^"
        let underline = match self.severity {
            Severity::Error => underline.red(),
            Severity::Warning => underline.yellow(),
        }
        .bold()
        .to_string();
        self.lines.underline = underline;
        self
    }
    pub fn indent(mut self) -> Self {
        let line_num_str = format!("{:>4}", self.line_num);
        let indent_len = 4;
        let indent_str = draw_line(" ", indent_len - 1);
        let dim_indent = |s: &str| format!(" {s} │  ").dimmed();
        self.lines = LineSet {
            overline: dim_indent(&indent_str).to_string(),
            line: format!("{}{}", dim_indent(&line_num_str), self.lines.line),
            underline: format!("{}{}", dim_indent(&indent_str), self.lines.underline),
        };
        self
    }
    pub fn finish(self) -> String {
        self.bold_range()
            .underline_range()
            .indent()
            .lines
            .to_string()
    }
}
