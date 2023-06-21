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
        assert_eq!("<html><head><title>XSLT in Rust</title></head><body><p>A simple document.</p></body></html>".to_string(),xslt::xml2html("<Example><Title>XSLT in Rust</Title><Paragraph>A simple document.</Paragraph></Example>","<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
        <xsl:template match='child::Example'><html><xsl:apply-templates/></html></xsl:template>
        <xsl:template match='child::Title'><head><title><xsl:apply-templates/></title></head></xsl:template>
        <xsl:template match='child::Paragraph'><body><p><xsl:apply-templates/></p></body></xsl:template>
      </xsl:stylesheet>").unwrap());
    }
}
