<script>
    import {Schema} from 'prosemirror-model';
    import {addMentionNodes, addTagNodes, getMentionsPlugin} from '../../lib/prosemirror-mentions';
    import {keymap} from "prosemirror-keymap";
    import {history, redo, undo} from "prosemirror-history";
    import {baseKeymap} from "prosemirror-commands";
    import ProsemirrorEditor from 'prosemirror-svelte';
    import {toPlainText} from 'prosemirror-svelte/state';
    import {EditorState, TextSelection} from "prosemirror-state";
    import {getChannelUsers, getCurrentChannelId} from "$lib/api.js";

    export let text = "";
    export let sendMessage;

    // import the core component

    // import helpers to work with prosemirror state

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

    export function send() {
        // TODO: temporarily disable editing
        sendMessage(toPlainText(editorState)).then(() => {
            clear();
        });
    }

    const corePlugins = [
        history(),
        keymap({"Mod-z": undo, "Mod-y": redo, "Mod-Shift-z": redo, "Enter": send}),
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
            setTimeout(async () => {
                if (type === 'mention') {
                    // TODO: get real suggestions
                    let users = await getChannelUsers(await getCurrentChannelId());
                    console.log(users);
                    let results = [];
                    for (let user of users) {
                        results.push({name: user.fullname, id: user.id, email: ""});
                    }
                    done(results);
                } else {
                    // TODO: remove
                    done([{tag: 'SOHS'}, {tag: 'Stanford Online High School'}, {tag: 'Stanford'}])
                }
            }, 0);
        },
        getSuggestionsHTML: (items, type) => {
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
