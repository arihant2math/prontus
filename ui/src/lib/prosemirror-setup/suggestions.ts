/**
 * IMPORTANT: outer div's "suggestion-item-list" class is mandatory. The plugin uses this class for querying.
 * IMPORTANT: inner div's "suggestion-item" class is mandatory too for the same reasons
 */
import {getChannelUsers, getCurrentChannelId} from "$lib/api";
import Fuse from "fuse.js";
import {getMentionsPlugin} from '$lib/prosemirror-mentions';


function getMentionSuggestionsHTML(items) {
    return '<div class="suggestion-item-list bg-white text-gray-900 dark:bg-slate-800 dark:text-white rounded-lg">' +
        items.map(i => '<div class="suggestion-item p-1">' + i.name + '</div>').join('') +
        '</div>';
}

function getTagSuggestionsHTML(items) {
    return '<div class="suggestion-item-list bg-white text-gray-900 dark:bg-slate-800 dark:text-white rounded-lg">' +
        items.map(i => '<div class="suggestion-item p-1">' + i.tag + '</div>').join('') +
        '</div>';
}

const mentionPlugin = getMentionsPlugin({
    getSuggestions: (type, text, done) => {
        setTimeout(async () => {
            if (type === 'mention') {
                let users = await getChannelUsers((await getCurrentChannelId()).id);
                users.push({
                    id: 0,
                    fullname: "everyone"
                });
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

export default mentionPlugin;
