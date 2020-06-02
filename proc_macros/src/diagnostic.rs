use proc_macro2::Span;
use std::convert::TryFrom;
use syn::export::TokenStream2;

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug, Clone)]
pub(crate) struct Diagnostic {
    level: DiagnosticLevel,
    message: String,
    span: Span,
    hints: Vec<Diagnostic>,
}

macro_rules! diagnostic_hint_fns {
    ($func:ident, $func_in:ident, $level:path) => {
        #[allow(unused)]
        pub fn $func(mut self, message: impl Into<String>) -> Diagnostic {
            self.hints.push(Diagnostic::new($level, message, self.span));
            self
        }

        #[allow(unused)]
        pub fn $func_in(mut self, message: impl Into<String>, span: Span) -> Diagnostic {
            self.hints.push(Diagnostic::new($level, message, span));
            self
        }
    };
}

impl Diagnostic {
    pub fn new(level: DiagnosticLevel, message: impl Into<String>, span: Span) -> Self {
        Self {
            level,
            message: message.into(),
            span,
            hints: vec![],
        }
    }

    diagnostic_hint_fns!(error, error_in, DiagnosticLevel::Error);
    diagnostic_hint_fns!(warn, warn_in, DiagnosticLevel::Warning);
    diagnostic_hint_fns!(note, note_in, DiagnosticLevel::Note);
    diagnostic_hint_fns!(help, help_in, DiagnosticLevel::Help);

    #[cfg(not(nightly_diagnostics))]
    pub fn emit(self) -> proc_macro2::TokenStream {
        use std::convert::TryInto;

        let err: Result<syn::parse::Error, ()> = self.try_into();
        if let Ok(err) = err {
            err.to_compile_error()
        } else {
            TokenStream2::new()
        }
    }

    #[cfg(nightly_diagnostics)]
    pub fn emit(self) -> proc_macro2::TokenStream {
        let diag: proc_macro::Diagnostic = self.into();
        diag.emit();
        TokenStream2::new()
    }

    fn single_error_message(&self) -> String {
        let prefix = match self.level {
            DiagnosticLevel::Warning => "warning: ",
            DiagnosticLevel::Note => "note: ",
            DiagnosticLevel::Help => "help: ",
            _ => "",
        };
        format!("{}{}", prefix, self.message)
    }
}

impl TryFrom<Diagnostic> for syn::parse::Error {
    type Error = ();

    fn try_from(diag: Diagnostic) -> Result<Self, ()> {
        if diag.level != DiagnosticLevel::Error {
            Err(())
        } else {
            let mut error = syn::parse::Error::new(diag.span, diag.message);
            for hint in diag.hints {
                error.combine(syn::parse::Error::new(
                    hint.span,
                    hint.single_error_message(),
                ));
            }

            Ok(error)
        }
    }
}

#[cfg(nightly_diagnostics)]
impl From<DiagnosticLevel> for proc_macro::Level {
    fn from(lvl: DiagnosticLevel) -> Self {
        match lvl {
            DiagnosticLevel::Error => proc_macro::Level::Error,
            DiagnosticLevel::Warning => proc_macro::Level::Warning,
            DiagnosticLevel::Note => proc_macro::Level::Note,
            DiagnosticLevel::Help => proc_macro::Level::Help,
        }
    }
}

#[cfg(nightly_diagnostics)]
impl From<Diagnostic> for proc_macro::Diagnostic {
    fn from(value: Diagnostic) -> Self {
        let diag =
            proc_macro::Diagnostic::spanned(value.span.unwrap(), value.level.into(), value.message);

        value
            .hints
            .into_iter()
            .fold(diag, |diag, hint| match hint.level {
                DiagnosticLevel::Error => diag.span_error(hint.span.unwrap(), hint.message),
                DiagnosticLevel::Warning => diag.span_warning(hint.span.unwrap(), hint.message),
                DiagnosticLevel::Note => diag.span_note(hint.span.unwrap(), hint.message),
                DiagnosticLevel::Help => diag.span_help(hint.span.unwrap(), hint.message),
            })
    }
}

pub(crate) trait DiagnosticCreator {
    /// Create a compiler error and return the code to be placed into code.
    fn error<T: Into<String>>(&self, message: T) -> Diagnostic;

    /// Create a compiler warning (only in nightly)
    fn warn<T: Into<String>>(&self, message: T) -> Diagnostic;

    /// Create a compiler note (only in nightly)
    fn note<T: Into<String>>(&self, message: T) -> Diagnostic;

    /// Create a compiler help message (only in nightly)
    fn help<T: Into<String>>(&self, message: T) -> Diagnostic;
}

impl DiagnosticCreator for proc_macro2::Span {
    fn error<T: Into<String>>(&self, message: T) -> Diagnostic {
        Diagnostic::new(DiagnosticLevel::Error, message, *self)
    }
    fn warn<T: Into<String>>(&self, message: T) -> Diagnostic {
        Diagnostic::new(DiagnosticLevel::Warning, message, *self)
    }
    fn note<T: Into<String>>(&self, message: T) -> Diagnostic {
        Diagnostic::new(DiagnosticLevel::Note, message, *self)
    }
    fn help<T: Into<String>>(&self, message: T) -> Diagnostic {
        Diagnostic::new(DiagnosticLevel::Help, message, *self)
    }
}
