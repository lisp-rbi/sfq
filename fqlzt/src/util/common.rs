// Trimm new line characer -> there is probably a better way to do this 
pub fn trim_nl(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
