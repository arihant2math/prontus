import {schema} from "prosemirror-schema-basic";
import {Schema} from "prosemirror-model";
import {addMentionNodes, addTagNodes} from '../../lib/prosemirror-mentions';

const customSchema = new Schema({
    nodes: addTagNodes(addMentionNodes(schema.spec.nodes)),
    marks: schema.spec.marks
});

export default customSchema;
