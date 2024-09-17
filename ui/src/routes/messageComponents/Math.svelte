<script>
    import katex from 'katex';

    export let latex = "";

    $: html = katex.renderToString(latex, {
        throwOnError: false
    });

    $: html, console.log(html);
</script>

{@html html}
