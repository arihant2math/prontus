<script>
    export let content;
</script>
<div>
    {#if content !== undefined}
        {#if content.type === "root"}
            {#each content.children as child}
                <svelte:self content={child} />
            {/each}
        {:else if content.type === "element"}
            {#if content.tagName === "blockquote"}
                <blockquote class="px-4 py-1 my-2 border-s-4 border-gray-300 bg-gray-50 dark:border-gray-500 dark:bg-gray-800 text-gray-900 dark:text-white">
                    {#each content.children as child}
                        <svelte:self content={child} />
                    {/each}
                </blockquote>
            {:else if content.tagName === "break"}
                <br>
            {:else if content.tagName === "p"}
                <p>
                    {#each content.children as child}
                        <svelte:self content={child} />
                    {/each}
                </p>
            {:else if content.tagName === "code"}
                <code>
                    {#each content.children as child}
                        <svelte:self content={child} />
                    {/each}
                </code>
            {:else if content.tagName === "em"}
                <i>
                    {#each content.children as child}
                        <svelte:self content={child} />
                    {/each}
                </i>
            {:else if content.tagName === "h1"}
                <h1 class="text-2xl">
                    {#each content.children as child}
                        <svelte:self content={child} />
                    {/each}
                </h1>
            {:else if content.tagName === "h2"}
                <h2 class="text-xl">
                    {#each content.children as child}
                        <svelte:self content={child} />
                    {/each}
                </h2>
            {:else if content.tagName === "h3"}
                <h3 class="text-lg">
                    {#each content.children as child}
                        <svelte:self content={child} />
                    {/each}
                </h3>
                <!--    HTML not implemented on purpose -->
            {:else if content.tagName === "inlineCode"}
                <code>
                    {content.value}
                </code>
            {:else if content.tagName === "strong"}
                <b>
                    {#each content.children as child}
                        <svelte:self content={child} />
                    {/each}
                </b>
            {:else if content.tagName === "del"}
                <del>
                    {#each content.children as child}
                        <svelte:self content={child} />
                    {/each}
                </del>
            {/if}
        {:else if content.type === "text"}
            {content.value}
        {:else if content.type === "link"}
            <a class="text-blue-600 dark:text-blue-400 hover:text-blue-500" href="{content.url}" target="_blank">
                {#each content.children as child}
                    <svelte:self content={child} />
                {/each}
            </a>
        {:else}
            <code class="text-red-500">{content.type}</code>
            {#if content.children !== undefined}
                {#each content.children as child}
                    <svelte:self content={child} />
                {/each}
            {/if}
        {/if}
    {/if}
</div>