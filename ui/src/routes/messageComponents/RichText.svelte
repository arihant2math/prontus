<script>
    import RichText from './RichText.svelte';
    import UserInfo from "../user/UserInfo.svelte";
    import ProfilePicture from "../user/ProfilePicture.svelte";
    import { Popover, Separator, Toggle } from "bits-ui";
    import Mention from "./Mention.svelte";

    /** @type {{content: any}} */
    let { content } = $props();

    if (content !== undefined) {
        if (content.type !== "root" && content.type !== "text") {
            if (!(content.hasOwnProperty("tagName"))) {
                console.log(content);
            }
        }
    }
</script>
{#if content !== undefined}
    {#if content.type === "root"}
        {#each content.children as child}
            <RichText content={child} />
        {/each}
    {:else if content.type === "element"}
        {#if content.tagName === "blockquote"}
            <blockquote class="px-4 py-1 my-2 border-s-4 border-gray-300 bg-gray-50 dark:border-gray-500 dark:bg-gray-800 text-gray-900 dark:text-white">
                {#each content.children as child}
                    <RichText content={child} />
                {/each}
            </blockquote>
        {:else if content.tagName === "break"}
            <br>
        {:else if content.tagName === "p"}
            <p>
                {#each content.children as child}
                    <RichText content={child} />
                {/each}
            </p>
        {:else if content.tagName === "code"}
            <code>
                {#each content.children as child}
                    <RichText content={child} />
                {/each}
            </code>
        {:else if content.tagName === "em"}
            <i>
                {#each content.children as child}
                    <RichText content={child} />
                {/each}
            </i>
        {:else if content.tagName === "h1"}
            <h1 class="text-2xl">
                {#each content.children as child}
                    <RichText content={child} />
                {/each}
            </h1>
        {:else if content.tagName === "h2"}
            <h2 class="text-xl">
                {#each content.children as child}
                    <RichText content={child} />
                {/each}
            </h2>
        {:else if content.tagName === "h3"}
            <h3 class="text-lg">
                {#each content.children as child}
                    <RichText content={child} />
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
                    <RichText content={child} />
                {/each}
            </b>
        {:else if content.tagName === "a"}
            {#if content.properties.href.startsWith("MENTION")}
                <Mention id={parseInt(content.properties.href.split('_')[1])}/>
            {:else}
                <a class="text-blue-600 dark:text-blue-400 hover:text-blue-500" href="{content.properties.href}" target="_blank">
                    {#each content.children as child}
                        <RichText content={child} />
                    {/each}
                </a>
            {/if}
        {:else if content.tagName === "del"}
            <del>
                {#each content.children as child}
                    <RichText content={child} />
                {/each}
            </del>
        {/if}
    {:else if content.type === "text"}
        {content.value}
    {:else if content.type === "link"}
        <a class="text-blue-600 dark:text-blue-400 hover:text-blue-500" href="{content.url}" target="_blank">
            {#each content.children as child}
                <RichText content={child} />
            {/each}
        </a>
    {:else if content.type === "mention"}
        <Popover.Root>
            <Popover.Trigger>
                <p class="text-nowrap text-blue-500">{content.user.fullname}</p>
            </Popover.Trigger>
            <!--    For popover content-->
            <!--            transition={flyAndScale}-->
            <Popover.Content
                    class="z-50 w-full max-w-max rounded-lg bg-white dark:bg-slate-800 shadow-md p-4"
                    sideOffset={8}>
<!--                <UserInfo user={user}/>-->
            </Popover.Content>
        </Popover.Root>
    {:else}
        <code class="text-red-500">{content.type}</code>
        {#if content.children !== undefined}
            {#each content.children as child}
                <RichText content={child} />
            {/each}
        {/if}
    {/if}
{/if}