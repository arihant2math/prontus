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

    export let text = "";
    export let sendMessage;
    let disabled = false;
    let props = {
        editable() {
            return !disabled;
        },
    };


    const doc = defaultMarkdownParser.parse(text);
    const selection = doc ? TextSelection.atEnd(doc) : undefined;

    export function send() {
        // TODO: temporarily disable editing
        let rawText = customMarkdownSerializer.serialize(editorState.doc);
        if (rawText === "") {
            return;
        }
        disabled = true;
        sendMessage(rawText).then(() => {
            clear();
            disabled = false;
        });
        clear();
        return true;
    }

    const corePlugins = [
        history(),
        keymap(buildKeymap(send)),
        keymap(baseKeymap),
    ];

    let editorState = EditorState.create({
        schema: customSchema,
        doc,
        selection,
        plugins: [
            mentionPlugin,
            ...corePlugins,
        ]
    });

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
</script>

<ProsemirrorEditor
        placeholder="Message"
        className="rounded-md p-2 bg-grey-100 dark:bg-slate-800"
        {editorState}
        on:change={handleChange}
/>
