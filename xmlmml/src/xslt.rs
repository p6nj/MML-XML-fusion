pub fn xslt2html() {
    use std::rc::Rc;
    use xrust::evaluate::{Evaluator, StaticContext};
    use xrust::intmuttree::{Document, NodeBuilder, RNode};
    use xrust::item::{Item, Node, NodeType, Sequence, SequenceTrait};
    use xrust::qname::QualifiedName;
    use xrust::xdmerror::Error;
    use xrust::xslt::from_document;

    // A little helper function that wraps the toplevel node in a Document
    fn make_from_str(s: &str) -> Result<RNode, Error> {
        let e = Document::try_from(s).expect("failed to parse XML").content[0].clone();
        let mut d = NodeBuilder::new(NodeType::Document).build();
        d.push(e).expect("unable to append node");
        Ok(d)
    }

    // First setup a static context for the evaluator
    let mut sc = StaticContext::new_with_builtins();

    // The source document (a tree)
    let src = Rc::new(Item::Node(
    make_from_str("<Example><Title>XSLT in Rust</Title><Paragraph>A simple document.</Paragraph></Example>")
    .expect("unable to parse XML")
));

    // The XSL stylesheet
    let style = make_from_str("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Example'><html><xsl:apply-templates/></html></xsl:template>
  <xsl:template match='child::Title'><head><title><xsl:apply-templates/></title></head></xsl:template>
  <xsl:template match='child::Paragraph'><body><p><xsl:apply-templates/></p></body></xsl:template>
</xsl:stylesheet>")
    .expect("unable to parse stylesheet");

    // Compile the stylesheet
    let ev = from_document(style, &mut sc, None, make_from_str, |_| Ok(String::new()))
        .expect("failed to compile stylesheet");

    // Make an empty result document
    let rd = NodeBuilder::new(NodeType::Document).build();

    // Prime the stylesheet evaluation by finding the template for the document root
    // and making the document root the initial context
    let t = ev
        .find_match(&src, None, &rd)
        .expect("unable to find match");

    // Let 'er rip!
    // Evaluate the sequence constructor with the source document as the initial context
    let seq = ev
        .evaluate(Some(vec![Rc::clone(&src)]), Some(0), &t, &rd)
        .expect("evaluation failed");
}
