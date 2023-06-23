#[allow(unused, dead_code)]
mod files;
#[allow(unused, dead_code)]
mod structure;
#[allow(unused, dead_code)]
mod xslt;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn xslt() {
        assert_eq!(
        "<html><head><title>XSLT in Rust</title></head><body><p>A simple document.</p></body></html>".to_string(),
        xslt::xml2html("<xml><Title>XSLT in Rust</Title><Paragraph>A simple document.</Paragraph></xml>").unwrap()
    );
    }
}
