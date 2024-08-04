pub(crate) struct Error<'a> {
    line: usize,
    location: &'a str,
    message: &'a str,
}

impl<'a> Error<'a> {
    pub(crate) fn new(line: usize, location: &'a str, message: &'a str) -> Error<'a> {
        Error {
            line,
            location,
            message,
        }
    }
    pub(crate) fn report(&self) {
        eprintln!(
            "[line {}] Error{}: {}",
            self.line, self.location, self.message
        );
    }
}
