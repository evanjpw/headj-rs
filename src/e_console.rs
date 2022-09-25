use console::{Style, Term};
use eyre::Result;
use once_cell::unsync::OnceCell;

pub struct EConsole {
    term: Term,
    quiet: bool,
    use_stderr: bool,
    debug: u8,
}

static E_CONSOLE: OnceCell<EConsole> = OnceCell::new();

impl EConsole {
    const ERROR: &'static str = "red";
    const WARNING: &'static str = "yellow";
    const INFO: &'static str = "green";
    const DEBUG: &'static str = "cyan";
    const TRACE: &'static str = "magenta";

    pub fn new(quiet: bool, debug: u8, use_stderr: Option<bool>) -> Self {
        let use_stderr = use_stderr.unwrap_or(true);
        let term = if use_stderr {
            Term::stderr()
        } else {
            Term::stdout()
        };
        Self {
            term,
            quiet,
            debug,
            use_stderr,
        }
    }

    pub fn init(quiet: bool, debug: u8, use_stderr: Option<bool>) -> Result<()> {
        let e_console = Self::new(quiet, debug, use_stderr);
        E_CONSOLE.set(e_console)?;
        Ok(())
    }
    pub fn console() -> &'static EConsole {
        E_CONSOLE.get().expect("E_CONSOLE is not initialized"
)
    }

    pub fn write_line_with_style(&self, s: &str, style_str: &str) -> Result<()> {
        let line_style = Style::from_dotted_str(style_str);
        let term_line_style = if self.use_stderr {
            line_style.for_stderr()
        } else {
            line_style.for_stdout()
        };
        self.write_line(term_line_style.apply_to(s).to_string().as_str())
    }

    pub fn write_line(&self, s: &str) -> Result<()> {
        if !self.quiet {
            self.term.write_line(s)?
        }
        Ok(())
    }

    pub fn error(&self, s: &str) -> Result<()> {
        self.write_line_with_style(s, Self::ERROR)
    }

    pub fn warning(&self, s: &str) -> Result<()> {
        self.write_line_with_style(s, Self::WARNING)
    }

    pub fn info(&self, s: &str) -> Result<()> {
        self.write_line_with_style(s, Self::INFO)
    }

    pub fn debug(&self, s: &str) -> Result<()> {
        if self.debug > 0 {
            self.write_line_with_style(s, Self::DEBUG)
        } else {
            Ok(())
        }
    }

    pub fn trace(&self, s: &str) -> Result<()> {
        if self.debug > 1 {
            self.write_line_with_style(s, Self::TRACE)
        } else {
            Ok(())
        }
    }
}

impl Default for EConsole {
    fn default() -> Self {
        Self::new(false, 0, None)
    }
}
