<script>
    import {keymap} from "prosemirror-keymap";
    import {history} from "prosemirror-history";
    import {baseKeymap} from "prosemirror-commands";
    import {defaultMarkdownParser} from 'prosemirror-markdown';
    import ProsemirrorEditor from 'prosemirror-svelte';
    import {EditorState, TextSelection} from "prosemirror-state";
    import customSchema from "$lib/prosemirror-setup/schema.ts"
    import mentionPlugin from "$lib/prosemirror-setup/suggestions.ts";
    import customMarkdownSerializer from "$lib/prosemirror-setup/markdown.ts";
    import {buildKeymap} from "$lib/prosemirror-setup/keymap.ts";

    /** @type {{text?: string, sendMessage: any, disabled: any}} */
    let {text = $bindable(""), sendMessage, disabled = false} = $props();
    let enabled = $state(false);

    let editorProps = {
        editable() {
            return enabled && !disabled;
        },
    };


    const doc = defaultMarkdownParser.parse(text);
    const selection = doc ? TextSelection.atEnd(doc) : undefined;

    export function send() {
        let rawText = customMarkdownSerializer.serialize(editorState.doc);
        if (rawText === "") {
            return;
        }
        enabled = false;
        sendMessage(rawText).then(() => {
            clear();
            enabled = true;
        });
        clear();
        return true;
    }

    const corePlugins = [
        history(),
        keymap(buildKeymap(send)),
        keymap(baseKeymap),
    ];

    let editorState = $state(EditorState.create({
        schema: customSchema,
        doc,
        selection,
        plugins: [
            mentionPlugin,
            ...corePlugins,
        ]
    }));

    function handleChange(event) {
        // get the new editor state from event.detail
        editorState = event.detail.editorState;
        text = customMarkdownSerializer.serialize(editorState.doc);
    }

    export function clear() {
        editorState = EditorState.create({
            schema: customSchema,
            doc: "" ? customSchema.node("doc", null, [
                customSchema.text("")
            ]) : undefined,
            selection: undefined,
            plugins: [
                mentionPlugin,
                ...corePlugins,
            ]
        });
    }

    let placeholder = $derived(disabled ? "You are not allowed to send messages" : "Message");
    let bgColor = $derived(enabled ? "bg-grey-100 dark:bg-slate-800" : "bg-grey-200 dark:bg-slate-700");
</script>

<ProsemirrorEditor
        placeholder={placeholder}
        className="rounded-md p-2 {bgColor}"
        {editorState}
        on:change={handleChange}
/>
