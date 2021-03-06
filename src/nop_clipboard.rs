pub struct ClipboardContext;

impl ClipboardContext {
    pub fn new() -> Result<ClipboardContext, &'static str> {
        Ok(ClipboardContext)
    }
    pub fn get_contents(&self) -> Result<String, &str> {
        println!("Attempting to get the contents of the clipboard, which hasn't yet been implemented on this platform.");
        Ok("".to_string())
    }
    pub fn set_contents(&self, _: String) -> Result<(), &str> {
        println!("Attempting to set the contents of the clipboard, which hasn't yet been implemented on this platform.");
        Ok(())
    }
}
