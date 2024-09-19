/**
 * @typedef {import('mdast').Root} Root
 *
 * @typedef {import('mdast').PhrasingContent} PhrasingContent
 *
 * @typedef {import('mdast-util-find-and-replace').ReplaceFunction} ReplaceFunction
 *
 * @typedef Options
 *  Configuration
 */

import {findAndReplace} from "mdast-util-find-and-replace";
import {getUser} from "$lib/api.ts";

const userGroup = "[\\da-z][-\\da-z_]{0,38}";
const mentionRegex = new RegExp(
    "(<@" + userGroup + ">)",
    "gi"
);

/**
 *
 * @type {import("unified").Plugin<[Options?]|void[], Root>}
 */
export default function remarkMentions(
    opts = {}
) {
    // @ts-ignore
    return (tree, _file) => {
        findAndReplace(tree, [[mentionRegex, replaceMention]]);
    };

    /**
     * @type {ReplaceFunction}
     * @param {string} value
     * @param {string} text
     */
    async function replaceMention(value, text) {
        /** @type {PhrasingContent[]} */
        let whitespace = [];

        let id = text.substring(2, text.length - 1);
        let user;
        if (id !== "everyone") {
            user = await getUser(parseInt(id));
        } else {
            user = {
                "fullname": "everyone"
            }
        }

        // Separate leading white space
        if (value.indexOf("@") > 0) {
            whitespace.push({
                type: "text",
                value: value.substring(0, value.indexOf("@")),
            });
        }

        return [
            ...whitespace,
            {
                type: "link",
                url: "#",
                children: [
                    {type: "strong", children: [{type: "text", value: user.fullname}]},
                ],
            },
        ];
    }
}
