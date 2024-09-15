<script>
    import RichTextContainer from "./RichTextContainer.svelte";
    import {Schema} from 'prosemirror-model';
    import {addMentionNodes, addTagNodes, getMentionsPlugin} from '../../lib/prosemirror-mentions';
    import {keymap} from "prosemirror-keymap";
    import {history, redo, undo} from "prosemirror-history";
    import {baseKeymap} from "prosemirror-commands";
    import {dropCursor} from "prosemirror-dropcursor"
    import {gapCursor} from "prosemirror-gapcursor"

    export let text = "";
    // import the core component
    import ProsemirrorEditor from 'prosemirror-svelte';

    // import helpers to work with prosemirror state
    import {createSingleLineEditor, toPlainText} from 'prosemirror-svelte/state';
    import {EditorState, TextSelection} from "prosemirror-state";

    // create the initial editor state
    const singleLineSchema = new Schema({
        nodes: {
            doc: {content: "text*"},
            text: {inline: true}
        }
    });

    let schema = new Schema({
        nodes: addTagNodes(addMentionNodes(singleLineSchema.spec.nodes)),
        marks: singleLineSchema.spec.marks
    });

    const doc = "" ? schema.node("doc", null, [
        schema.text("")
    ]) : undefined;
    const selection = doc ? TextSelection.atEnd(doc) : undefined;

    const corePlugins = [
        history(),
        keymap({"Mod-z": undo, "Mod-y": redo, "Mod-Shift-z": redo}),
        keymap(baseKeymap),
    ];

    const getMentionSuggestionsHTML = items => '<div class="suggestion-item-list">' +
        items.map(i => '<div class="suggestion-item">' + i.name + '</div>').join('') +
        '</div>';

    /**
     * IMPORTANT: outer div's "suggestion-item-list" class is mandatory. The plugin uses this class for querying.
     * IMPORTANT: inner div's "suggestion-item" class is mandatory too for the same reasons
     */
    const getTagSuggestionsHTML = items => '<div class="suggestion-item-list">' +
        items.map(i => '<div class="suggestion-item">' + i.tag + '</div>').join('') +
        '</div>';

    let mentionPlugin = getMentionsPlugin({
        getSuggestions: (type, text, done) => {
            console.log("ent")
            setTimeout(() => {
                if (type === 'mention') {
                    // pass dummy mention suggestions
                    done([{name: 'John Doe', id: '101', email: 'joe@gmail.com'}, {
                        name: 'Joe Lewis',
                        id: '102',
                        email: 'lewis@gmail.com'
                    }])
                } else {
                    // pass dummy tag suggestions
                    done([{tag: 'WikiLeaks'}, {tag: 'NetNeutrality'}])
                }
            }, 0);
        },
        getSuggestionsHTML: (items, type) => {
            console.log("h ent")
            if (type === 'mention') {
                return getMentionSuggestionsHTML(items)
            } else if (type === 'tag') {
                return getTagSuggestionsHTML(items)
            }
        }
    });

    let editorState = EditorState.create({
        schema: schema,
        doc,
        selection,
        plugins: [
            mentionPlugin,
            ...corePlugins,
        ]
    });

    function handleChange(event) {
        console.log(event);
        // get the new editor state from event.detail
        editorState = event.detail.editorState;
        text = toPlainText(editorState);
    }

    $: console.log(toPlainText(editorState));

    export function clear() {
        editorState = EditorState.create({
            schema: schema,
            doc: "" ? schema.node("doc", null, [
                schema.text("")
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
        {editorState}
        on:change={handleChange}
/>
