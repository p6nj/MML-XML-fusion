<xsl:stylesheet version="1.0" xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
    <xsl:template match='child::xml'><html>
            <xsl:apply-templates />
        </html></xsl:template>
    <xsl:template match='child::Title'><head>
            <title>
                <xsl:apply-templates />
            </title>
        </head></xsl:template>
    <xsl:template match='child::Paragraph'><body>
            <p>
                <xsl:apply-templates />
            </p>
        </body></xsl:template>
</xsl:stylesheet>