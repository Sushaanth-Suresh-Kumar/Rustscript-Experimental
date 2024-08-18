use super::token::Token;
use crate::TokenKind;
use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};

#[derive(Debug)]
pub(crate) struct LErrorHandler<'a> {
    code: &'a str,
    file_name: &'a str,
}

impl<'a> LErrorHandler<'a> {
    pub(crate) fn new(code: &'a str, file_name: &'a str) -> LErrorHandler<'a> {
        LErrorHandler { code, file_name }
    }

    pub(crate) fn error_report(&self, err_token: &Token) {
        let mut colors = ColorGenerator::new();

        // Generate & choose some colours for each of our elements
        let a = colors.next();
        let out = Color::Fixed(81);

        match err_token.kind {
            TokenKind::InvalidIdent => {
                Report::build(ReportKind::Error, self.file_name, err_token.span.low)
                    .with_code(1)
                    .with_message(format!("Invalid Identifier"))
                    .with_label(
                        Label::new((self.file_name, err_token.span.low..err_token.span.high))
                            .with_message(format!("This is an {} identifier name", "invalid".fg(a)))
                            .with_color(a),
                    )
                    .with_note(format!(
                        "Identifier names must be made of {} or {} characters which are not {}",
                        "ascii".fg(out),
                        "unicode".fg(out),
                        "emoji".fg(out)
                    ))
                    .finish()
                    .print((self.file_name, Source::from(self.code)))
                    .unwrap()
            }
            TokenKind::InvalidDecimal => {
                Report::build(ReportKind::Error, self.file_name, err_token.span.low)
                    .with_code(2)
                    .with_message(format!("Invalid Floating Point"))
                    .with_label(
                        Label::new((self.file_name, err_token.span.low..err_token.span.high))
                            .with_message(format!(
                                "No digits found after {} point",
                                "decimal".fg(a)
                            ))
                            .with_color(a),
                    )
                    .with_note(format!(
                        "Atleast one {} must be present after {} point",
                        "digit".fg(out),
                        "decimal".fg(out)
                    ))
                    .finish()
                    .print((self.file_name, Source::from(self.code)))
                    .unwrap()
            }
            TokenKind::InvalidExponent => {
                Report::build(ReportKind::Error, self.file_name, err_token.span.low)
                    .with_code(3)
                    .with_message(format!("Invalid Exponent"))
                    .with_label(
                        Label::new((self.file_name, err_token.span.low..err_token.span.high))
                            .with_message(format!("No digits found after {}", "exponent".fg(a)))
                            .with_color(a),
                    )
                    .with_note(format!(
                        "Atleast one {} must be present after {}",
                        "digit".fg(out),
                        "exponent".fg(out)
                    ))
                    .finish()
                    .print((self.file_name, Source::from(self.code)))
                    .unwrap()
            }
            TokenKind::BlockComment { terminated: false } => {
                Report::build(ReportKind::Error, self.file_name, err_token.span.low)
                    .with_code(4)
                    .with_message(format!("Unterminated Block Comment"))
                    .with_label(
                        Label::new((self.file_name, err_token.span.low..err_token.span.high))
                            .with_message(format!("block {} is unterminated", "comment".fg(a)))
                            .with_color(a),
                    )
                    .with_note(format!(
                        "block {} must be terminated with {}",
                        "comment".fg(out),
                        "*/".fg(out)
                    ))
                    .finish()
                    .print((self.file_name, Source::from(self.code)))
                    .unwrap()
            }
            TokenKind::Unknown => {
                Report::build(ReportKind::Error, self.file_name, err_token.span.low)
                    .with_code(5)
                    .with_message(format!("Unknown Token"))
                    .with_label(
                        Label::new((self.file_name, err_token.span.low..err_token.span.high))
                            .with_message(format!("This is an {} token", "unknown".fg(a)))
                            .with_color(a),
                    )
                    .with_note(format!(
                        "This {} does not belong to the {}",
                        "token".fg(out),
                        "language".fg(out)
                    ))
                    .finish()
                    .print((self.file_name, Source::from(self.code)))
                    .unwrap()
            }
            _ => {}
        }
    }
}
