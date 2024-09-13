<script>
    export let content;
</script>
{#if content !== undefined}
    {#if content.data.t === "Document"}
        {#each content.children as child}
            <svelte:self content={child} />
        {/each}
    {:else if content.data.t === "Paragraph"}
        <p class="text-sm font-normal text-gray-900 dark:text-white">
            {#each content.children as child}
                <svelte:self content={child} />
            {/each}
        </p>
    {:else if content.data.t === "Text"}
        {content.data.c}
    {:else if content.data.t === "Emph"}
        <em>
            {#each content.children as child}
                <svelte:self content={child} />
            {/each}
        </em>
    {:else if content.data.t === "Strong"}
        <strong>
            {#each content.children as child}
                <svelte:self content={child} />
            {/each}
        </strong>
    {:else if content.data.t === "Strikethrough"}
        <del>
            {#each content.children as child}
                <svelte:self content={child} />
            {/each}
        </del>
    {:else if content.data.t === "Superscript"}
        <sup>
            {#each content.children as child}
                <svelte:self content={child} />
            {/each}
        </sup>
    {:else if content.data.t === "Subscript"}
        <sub>
            {#each content.children as child}
                <svelte:self content={child} />
            {/each}
        </sub>
    {:else if content.data.t === "Link"}
        <a href="{content.data.c.url}">{content.data.c.title}</a>
    {:else if content.data.t === "Image"}
        Markdown Images are not currently supported
    {:else if content.data.t === "SoftBreak"}
        <br>
    {:else}
        <code>{content.data.t} is not supported at the moment, please file a bug</code>
    {/if}

<!--    /// Non-Markdown front matter.  Treated as an opaque blob.-->
<!--    FrontMatter(String),-->

<!--    /// **Block**. A [block quote](https://github.github.com/gfm/#block-quotes).  Contains other-->
<!--    /// **blocks**.-->
<!--    ///-->
<!--    /// ``` md-->
<!--    /// > A block quote.-->
<!--    /// ```-->
<!--    BlockQuote,-->

<!--    /// **Block**.  A [list](https://github.github.com/gfm/#lists).  Contains-->
<!--    /// [list items](https://github.github.com/gfm/#list-items).-->
<!--    ///-->
<!--    /// ``` md-->
<!--    /// * An unordered list-->
<!--    /// * Another item-->
<!--    ///-->
<!--    /// 1. An ordered list-->
<!--    /// 2. Another item-->
<!--    /// ```-->
<!--    List(NodeList),-->
<!---->
<!--    /// **Block**.  A [list item](https://github.github.com/gfm/#list-items).  Contains other-->
<!--    /// **blocks**.-->
<!--    Item(NodeList),-->
<!---->
<!--    /// **Block**. A description list, enabled with `ext_description_lists` option.  Contains-->
<!--    /// description items.-->
<!--    ///-->
<!--    /// It is required to put a blank line between terms and details.-->
<!--    ///-->
<!--    /// ``` md-->
<!--    /// Term 1-->
<!--    ///-->
<!--    /// : Details 1-->
<!--    ///-->
<!--    /// Term 2-->
<!--    ///-->
<!--    /// : Details 2-->
<!--    /// ```-->
<!--    DescriptionList,-->
<!---->
<!--    /// *Block**. An item of a description list.  Contains a term and one details block.-->
<!--    DescriptionItem(NodeDescriptionItem),-->
<!---->
<!--    /// **Block**. Term of an item in a definition list.-->
<!--    DescriptionTerm,-->
<!---->
<!--    /// **Block**. Details of an item in a definition list.-->
<!--    DescriptionDetails,-->
<!---->
<!--    /// **Block**. A code block; may be [fenced](https://github.github.com/gfm/#fenced-code-blocks)-->
<!--    /// or [indented](https://github.github.com/gfm/#indented-code-blocks).  Contains raw text-->
<!--    /// which is not parsed as Markdown, although is HTML escaped.-->
<!--    CodeBlock(NodeCodeBlock),-->
<!---->
<!--    /// **Block**. A [HTML block](https://github.github.com/gfm/#html-blocks).  Contains raw text-->
<!--    /// which is neither parsed as Markdown nor HTML escaped.-->
<!--    HtmlBlock(NodeHtmlBlock),-->
<!---->
<!---->
<!--    /// **Block**. A heading; may be an [ATX heading](https://github.github.com/gfm/#atx-headings)-->
<!--    /// or a [setext heading](https://github.github.com/gfm/#setext-headings). Contains-->
<!--    /// **inlines**.-->
<!--    Heading(NodeHeading),-->
<!---->
<!--    /// **Block**. A [thematic break](https://github.github.com/gfm/#thematic-breaks).  Has no-->
<!--    /// children.-->
<!--    ThematicBreak,-->
<!---->
<!--    /// **Block**. A footnote definition.  The `String` is the footnote's name.-->
<!--    /// Contains other **blocks**.-->
<!--    FootnoteDefinition(NodeFootnoteDefinition),-->
<!---->
<!--    /// **Block**. A [table](https://github.github.com/gfm/#tables-extension-) per the GFM spec.-->
<!--    /// Contains table rows.-->
<!--    Table(NodeTable),-->
<!---->
<!--    /// **Block**. A table row.  The `bool` represents whether the row is the header row or not.-->
<!--    /// Contains table cells.-->
<!--    TableRow(bool),-->
<!---->
<!--    /// **Block**.  A table cell.  Contains **inlines**.-->
<!--    TableCell,-->
<!---->
<!--    /// **Inline**.  [Textual content](https://github.github.com/gfm/#textual-content).  All text-->
<!--    /// in a document will be contained in a `Text` node.-->
<!--    Text(String),-->
<!---->
<!--    /// **Block**. [Task list item](https://github.github.com/gfm/#task-list-items-extension-).-->
<!--    /// The value is the symbol that was used in the brackets to mark a task item as checked, or-->
<!--    /// None if the item is unchecked.-->
<!--    TaskItem(Option<char>),-->
<!---->
<!--    /// **Inline**.  A [soft line break](https://github.github.com/gfm/#soft-line-breaks).  If-->
<!--    /// the `hardbreaks` option is set in `Options` during formatting, it will be formatted-->
<!--    /// as a `LineBreak`.-->
<!--    SoftBreak,-->
<!---->
<!--    /// **Inline**.  A [hard line break](https://github.github.com/gfm/#hard-line-breaks).-->
<!--    LineBreak,-->
<!---->
<!--    /// **Inline**.  A [code span](https://github.github.com/gfm/#code-spans).-->
<!--    Code(NodeCode),-->
<!---->
<!--    /// **Inline**.  [Raw HTML](https://github.github.com/gfm/#raw-html) contained inline.-->
<!--    HtmlInline(String),-->
<!---->
<!--    /// **Inline**.  [Emphasized](https://github.github.com/gfm/#emphasis-and-strong-emphasis)-->
<!--    /// text.-->
<!--    Emph,-->
<!---->
<!--    /// **Inline**.  [Strong](https://github.github.com/gfm/#emphasis-and-strong-emphasis) text.-->
<!--    Strong,-->
<!---->
<!--    /// **Inline**.  [Strikethrough](https://github.github.com/gfm/#strikethrough-extension-) text-->
<!--    /// per the GFM spec.-->
<!--    Strikethrough,-->
<!---->
<!--    /// **Inline**.  Superscript.  Enabled with `ext_superscript` option.-->
<!--    Superscript,-->
<!---->
<!--    /// **Inline**.  A [link](https://github.github.com/gfm/#links) to some URL, with possible-->
<!--    /// title.-->
<!--    Link(NodeLink),-->
<!---->
<!--    /// **Inline**.  An [image](https://github.github.com/gfm/#images).-->
<!--    Image(NodeLink),-->
<!---->
<!--    /// **Inline**.  A footnote reference.-->
<!--    FootnoteReference(NodeFootnoteReference),-->
<!---->
<!--    #[cfg(feature = "shortcodes")]-->
<!--    /// **Inline**. An Emoji character generated from a shortcode. Enable with feature "shortcodes".-->
<!--    ShortCode(NodeShortCode),-->
<!---->
<!--    /// **Inline**. A math span. Contains raw text which is not parsed as Markdown.-->
<!--    /// Dollar math or code math-->
<!--    ///-->
<!--    /// Inline math $1 + 2$ and $`1 + 2`$-->
<!--    ///-->
<!--    /// Display math $$1 + 2$$ and-->
<!--    /// $$-->
<!--    /// 1 + 2-->
<!--    /// $$-->
<!--    ///-->
<!--    Math(NodeMath),-->
<!---->
<!--    /// **Block**. A [multiline block quote](https://github.github.com/gfm/#block-quotes).  Spans multiple-->
<!--    /// lines and contains other **blocks**.-->
<!--    ///-->
<!--    /// ``` md-->
<!--    /// >>>-->
<!--    /// A paragraph.-->
<!--    ///-->
<!--    /// - item one-->
<!--    /// - item two-->
<!--    /// >>>-->
<!--    /// ```-->
<!--    MultilineBlockQuote(NodeMultilineBlockQuote),-->
<!---->
<!--    /// **Inline**.  A character that has been [escaped](https://github.github.com/gfm/#backslash-escapes)-->
<!--    Escaped,-->
<!---->
<!--    /// **Inline**.  A wikilink to some URL.-->
<!--    WikiLink(NodeWikiLink),-->
<!---->
<!--    /// **Inline**.  Underline. Enabled with `underline` option.-->
<!--    Underline,-->
<!---->
<!--    /// **Inline**.  Spoilered text.  Enabled with `spoiler` option.-->
<!--    SpoileredText,-->
<!---->
<!--    /// **Inline**. Text surrounded by escaped markup. Enabled with `spoiler` option.-->
<!--    /// The `String` is the tag to be escaped.-->
<!--    EscapedTag(String),-->
{/if}
