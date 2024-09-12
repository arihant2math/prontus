<script>
    import RichTextContainer from "./RichTextContainer.svelte";

    export let text = "";

    function keyDown(event) {
        text = document.getElementById("temp").innerHtml;
        console.log(text);
    }
</script>

<div id="temp" class="text-transparent rounded-lg bg-gray-100 dark:bg-slate-600 h-10 m-2" tabindex="0" contenteditable="true" onkeydown={keyDown}>
</div>
<RichTextContainer bind:message={text}/>