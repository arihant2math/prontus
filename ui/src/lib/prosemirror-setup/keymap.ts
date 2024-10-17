import {redo, undo} from "prosemirror-history";
import {undoInputRule} from "prosemirror-inputrules";
import {
    chainCommands,
    createParagraphNear,
    joinDown,
    joinUp,
    lift, liftEmptyBlock,
    newlineInCode,
    selectParentNode, splitBlock, toggleMark
} from "prosemirror-commands";
import {schema} from "prosemirror-schema-basic";

export function buildKeymap(sendFunction) {
    let keys = {}, type

    function bind(key: string, cmd) {
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
    bind("Enter", sendFunction)
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