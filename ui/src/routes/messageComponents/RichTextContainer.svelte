<script>
    import {remark} from "remark";
    import remarkGfm from "remark-gfm";
    import remarkMentions from "../../lib/remark-mentions.js"
    import {unified} from 'unified';
    import rehypeSanitize from 'rehype-sanitize'
    import rehypeStringify from 'rehype-stringify'
    import remarkParse from 'remark-parse'
    import remarkRehype from 'remark-rehype'
    import remarkMath from 'remark-math'
    import rehypeKatex from 'rehype-katex'

    export let message;
    let richContent;
    $: message, init();

    function init() {
        let rich = unified()
            .use(remarkGfm)
            .use(remarkMentions)
            .use(remarkParse)
            .use(remarkMath)
            .use(remarkRehype)
            .use(rehypeSanitize)
            .use(rehypeKatex)
            .use(rehypeStringify)
            .process(message);
        rich.then((content) => {
            // console.log(content);
            richContent = content;
        });
    }

    init();
</script>
{#if richContent === undefined}
    <p>Loading...</p>
{:else}
    {@html richContent}
{/if}
