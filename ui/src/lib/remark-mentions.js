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
    function replaceMention(value, text) {
        /** @type {PhrasingContent[]} */
        console.log(value, text);
        let whitespace = [];

        let id = text.substring(2, text.length - 1);
        let user;
        if (id !== "everyone") {
            user = {
                "id": id,
                "fullname": "unknown"
            };
        } else {
            user = {
                "id": 0,
                "fullname": "everyone"
            }
        }

        // Separate leading white space
        // TODO: this is messy atm due to bad regex
        if (value.indexOf("<") > 0) {
            whitespace.push({
                type: "text",
                value: value.substring(0, value.indexOf("<")),
            });
        }

        console.log(user);

        return [
            ...whitespace,
            {
                type: "link",
                url: "MENTION_" + user.id,
                user: user,
                children: [
                    {type: "strong", children: [{type: "text", value: "@" + user.fullname}]},
                ],
            },
        ];
    }
}
