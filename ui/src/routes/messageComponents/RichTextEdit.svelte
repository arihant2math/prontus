<script>
    import {Schema} from 'prosemirror-model';
    import {addMentionNodes, addTagNodes, getMentionsPlugin} from '../../lib/prosemirror-mentions';
    import {keymap} from "prosemirror-keymap";
    import {history, redo, undo} from "prosemirror-history";
    import {
        baseKeymap,
        chainCommands, createParagraphNear,
        joinDown,
        joinUp,
        lift, liftEmptyBlock, newlineInCode,
        selectParentNode, splitBlock,
        toggleMark
    } from "prosemirror-commands";
    import ProsemirrorEditor from 'prosemirror-svelte';
    import {toPlainText} from 'prosemirror-svelte/state';
    import {schema} from 'prosemirror-schema-basic';
    import {EditorState, TextSelection} from "prosemirror-state";
    import {undoInputRule} from "prosemirror-inputrules"
    import {getChannelUsers, getCurrentChannelId} from "$lib/api.ts";
    import Fuse from 'fuse.js'
    import {defaultMarkdownSerializer} from "prosemirror-markdown";

    export let text = "";
    export let sendMessage;

    // import the core component

    // import helpers to work with prosemirror state

    // create the initial editor state
    const singleLineSchema = new Schema({
        nodes: schema.spec.nodes,
        marks: schema.spec.marks
    });

    let customSchema = new Schema({
        nodes: addTagNodes(addMentionNodes(singleLineSchema.spec.nodes)),
        marks: singleLineSchema.spec.marks
    });

    const doc = "" ? customSchema.node("doc", null, [
        customSchema.text("")
    ]) : undefined;
    const selection = doc ? TextSelection.atEnd(doc) : undefined;

    export function send() {
        // TODO: temporarily disable editing
        sendMessage(defaultMarkdownSerializer.serialize(editorState.doc)).then(() => {
            clear();
        });
    }

    function buildKeymap() {
        let keys = {}, type

        function bind(key, cmd) {
            keys[key] = cmd;
        }

        bind("Mod-z", undo)
        bind("Shift-Mod-z", redo)
        bind("Backspace", undoInputRule)
        // if (!mac) bind("Mod-y", redo)

        bind("Alt-ArrowUp", joinUp)
        bind("Alt-ArrowDown", joinDown)
        bind("Mod-BracketLeft", lift)
        bind("Escape", selectParentNode)
        bind("Enter", send)
        bind("Shift-Enter", chainCommands(newlineInCode, createParagraphNear, liftEmptyBlock, splitBlock));

        if (type = schema.marks.strong) {
            bind("Mod-b", toggleMark(type))
            bind("Mod-B", toggleMark(type))
        }
        if (type = schema.marks.em) {
            bind("Mod-i", toggleMark(type))
            bind("Mod-I", toggleMark(type))
        }
        if (type = schema.marks.code)
            bind("Mod-`", toggleMark(type))

        return keys;
    }

    const corePlugins = [
        history(),
        keymap(buildKeymap()),
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
                    let users = await getChannelUsers(await getCurrentChannelId());
                    const fuse = new Fuse(users, {
                        keys: ['fullname', 'username']
                    });
                    let filteredUsers = fuse.search(text);
                    let results = [];
                    let maxResults = 5;
                    for (let user of filteredUsers) {
                        results.push({name: user.item.fullname, id: user.item.id, email: ""});
                        maxResults--;
                        if (maxResults === 0) {
                            break;
                        }
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
        text = toPlainText(editorState);
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
        {editorState}
        on:change={handleChange}
/>
