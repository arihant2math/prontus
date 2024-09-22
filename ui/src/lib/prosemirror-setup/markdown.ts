import {MarkdownSerializer, defaultMarkdownSerializer} from "prosemirror-markdown";


let defaultMarkdownSerializerNodes = {
    blockquote(state, node) {
        state.wrapBlock("> ", null, node, () => state.renderContent(node))
    },
    code_block(state, node) {
        // Make sure the front matter fences are longer than any dash sequence within it
        const backticks = node.textContent.match(/`{3,}/gm)
        const fence = backticks ? (backticks.sort().slice(-1)[0] + "`") : "```"

        state.write(fence + (node.attrs.params || "") + "\n")
        state.text(node.textContent, false)
        // Add a newline to the current content before adding closing marker
        state.write("\n")
        state.write(fence)
        state.closeBlock(node)
    },
    heading(state, node) {
        state.write(state.repeat("#", node.attrs.level) + " ")
        state.renderInline(node, false)
        state.closeBlock(node)
    },
    horizontal_rule(state, node) {
        state.write(node.attrs.markup || "---")
        state.closeBlock(node)
    },
    bullet_list(state, node) {
        state.renderList(node, "  ", () => (node.attrs.bullet || "*") + " ")
    },
    ordered_list(state, node) {
        let start = node.attrs.order || 1
        let maxW = String(start + node.childCount - 1).length
        let space = state.repeat(" ", maxW + 2)
        state.renderList(node, space, i => {
            let nStr = String(start + i)
            return state.repeat(" ", maxW - nStr.length) + nStr + ". "
        })
    },
    list_item(state, node) {
        state.renderContent(node)
    },
    paragraph(state, node) {
        state.renderInline(node)
        state.closeBlock(node)
    },

    image(state, node) {
        state.write("![" + state.esc(node.attrs.alt || "") + "](" + node.attrs.src.replace(/[\(\)]/g, "\\$&") +
            (node.attrs.title ? ' "' + node.attrs.title.replace(/"/g, '\\"') + '"' : "") + ")")
    },
    hard_break(state, node, parent, index) {
        for (let i = index + 1; i < parent.childCount; i++)
            if (parent.child(i).type != node.type) {
                state.write("\\\n")
                return
            }
    },
    mention(state, node) {
        console.log(node);
        state.write("<@");
        if (node.attrs.id === 0) {
            state.write("everyone")
        } else {
            state.write(node.attrs.id);
        }
        state.write(">");
    },
    text(state, node) {
        state.text(node.text!, !state.inAutolink)
    }
};

const customMarkdownSerializer = new MarkdownSerializer(
    defaultMarkdownSerializerNodes,
    defaultMarkdownSerializer.marks);

export default customMarkdownSerializer;
