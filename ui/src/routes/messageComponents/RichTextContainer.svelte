<script>
    import { run } from 'svelte/legacy';

    import {remark} from "remark";
    import remarkGfm from "remark-gfm";
    import remarkMentions from "$lib/remark-mentions.ts"
    import {unified} from 'unified';
    import rehypeSanitize from 'rehype-sanitize'
    import rehypeStringify from 'rehype-stringify'
    import remarkParse from 'remark-parse'
    import remarkRehype from 'remark-rehype'
    import remarkMath from 'remark-math'
    import rehypeKatex from 'rehype-katex'
    import {removePosition} from 'unist-util-remove-position'
    import RichText from "./RichText.svelte";

    /** @type {{message: any}} */
    let { message } = $props();
    let richContent = $state();

    function init() {
        let processor = unified()
            .use(remarkGfm)
            .use(remarkMentions)
            .use(remarkParse)
            .use(remarkMath)
            .use(remarkRehype)
            .use(rehypeSanitize)
            .use(rehypeKatex)
            .use(rehypeStringify);
        let parseTree = processor.parse(message);
        processor.run(parseTree).then((content) => {
            removePosition(content, {force: true})
            richContent = content;
        });
    }

    init();
    run(() => {
        message, init();
    });
</script>
{#if richContent === undefined}
    <p>Loading...</p>
{:else}
    <RichText content={richContent}/>
{/if}
