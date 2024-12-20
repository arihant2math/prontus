import {invoke} from "@tauri-apps/api/core";
import {toast} from "svelte-sonner";

export async function getCode(email: string): Promise<void> {
    try {
        return await invoke("get_code", {email});
    } catch (e) {
        toast.error("Error getting code", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function sendCode(email: string, code: string): Promise<void> {
    try {
        return await invoke("send_code", {email, code});
    } catch (e) {
        toast.error("Error sending code", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function load(): Promise<void> {
    try {
        return await invoke("load");
    } catch (e) {
        toast.error("Error loading", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getCurrentUser() {
    try {
        return await invoke("get_current_user");
    } catch (e) {
        toast.error("Error getting current user", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getUser(id: number) {
    try {
        return await invoke("get_user", {id});
    } catch (e) {
        toast.error("Error getting user", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function loadChannel(id: number): Promise<void> {
    try {
        await invoke("load_channel", {id});
    } catch (e) {
        toast.error("Error loading channel", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getChannelList() {
    try {
        return await invoke("get_channel_list");
    } catch (e) {
        toast.error("Error getting channel list", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getChannelInfo() {
    try {
        return await invoke("get_channel_info");
    } catch (e) {
        toast.error("Error getting channel info", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getMessage(id: number) {
    try {
        return await invoke("get_message", {id});
    } catch (e) {
        toast.error("Error getting message", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getMessages(): Promise<any[]> {
    try {
        return await invoke("get_messages");
    } catch (e) {
        toast.error("Error getting messages", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getMoreMessages(lastMessageId: number): Promise<any[]> {
    try {
        return await invoke("get_more_messages", {lastMessageId});
    } catch (e) {
        toast.error("Error getting more messages", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getParentMessages(): Promise<any[]> {
    try {
        return await invoke("get_parent_messages");
    } catch (e) {
        toast.error("Error getting parent messages", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function loadMessages(): Promise<void> {
    try {
        return await invoke("load_messages");
    } catch (e) {
        toast.error("Error loading messages", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function editMessage(messageId: number, message: String) {
    try {
        return await invoke("edit_message", {messageId, message});
    } catch (e) {
        toast.error("Error editing message", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function sendMessage(message: string, thread?: number) {
    try {
        return await invoke("send_message", {message, thread});
    } catch (e) {
        toast.error("Error sending message", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function deleteMessage(messageId: number) {
    try {
        return await invoke("delete_message", {messageId});
    } catch (e) {
        toast.error("Error deleting message", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function setReactionState(messageId: number, reactionId: number, active: boolean) {
    try {
        return await invoke("set_reaction_state", {messageId, reactionId, active});
    } catch (e) {
        toast.error("Error setting reaction state", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getChannelUsers(id: number): Promise<any> {
    try {
        return await invoke("get_channel_users", {id});
    } catch (e) {
        toast.error("Error getting channel users", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function loadChannelUsers(id: number): Promise<void> {
    try {
        return await invoke("load_channel_users", {id});
    } catch (e) {
        toast.error("Error loading channel users", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getSettings() {
    try {
        return await invoke("get_settings");
    } catch (e) {
        toast.error("Error getting settings", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function setSettings(settings: any): Promise<void> {
    try {
        return await invoke("set_settings", {settings});
    } catch (e) {
        toast.error("Error setting settings", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getCurrentChannelId(): Promise<any> {
    try {
        return await invoke("get_current_channel");
    } catch (e) {
        toast.error("Error getting current channel ID", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function setChannelMute(channelId: number, mute: boolean) {
    try {
        return await invoke("set_channel_mute", {channelId, mute});
    } catch (e) {
        toast.error("Error setting channel mute", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function setChannelPin(channelId: number, pin: boolean) {
    try {
        return await invoke("set_channel_pin", {channelId, pin});
    } catch (e) {
        toast.error("Error setting channel pin", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function setChannelAlias(channelId: number, alias?: string) {
    try {
        return await invoke("set_channel_alias", {channelId, alias});
    } catch (e) {
        toast.error("Error setting channel alias", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function setChannelTitle(channelId: number, title: string) {
    try {
        return await invoke("set_channel_title", {channelId, title});
    } catch (e) {
        toast.error("Error setting channel title", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function setChannelCategory(channelId: number, categoryId: number) {
    try {
        return await invoke("set_channel_category", {channelId, categoryId});
    } catch (e) {
        toast.error("Error setting channel category", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function modifyChannelPermission(channelId: number, key: string, value: string) {
    try {
        return await invoke("modify_channel_permission", {channelId, key, value});
    } catch (e) {
        toast.error("Error modifying channel permission", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function deleteChannel(channelId: number) {
    try {
        return await invoke("delete_channel", {channelId});
    } catch (e) {
        toast.error("Error deleting channel", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function setChannelNotifications(channelId: number, level: string) {
    try {
        return await invoke("set_channel_notifications", {channelId, level});
    } catch (e) {
        toast.error("Error setting channel notifications", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function readChannel(channelId: number) {
    try {
        return await invoke("read_channel", {channelId});
    } catch (e) {
        toast.error("Error reading channel", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function createDm(userId: number) {
    try {
        return await invoke("create_dm", {userId});
    } catch (e) {
        toast.error("Error creating DM", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function userSearch(query: string) {
    try {
        return await invoke("user_search", {query});
    } catch (e) {
        toast.error("Error searching user", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getAnnouncements() {
    try {
        return await invoke("get_announcements");
    } catch (e) {
        toast.error("Error getting announcements", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getTasks() {
    try {
        return await invoke("get_tasks");
    } catch (e) {
        toast.error("Error getting tasks", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function completeTask(taskId: number) {
    try {
        return await invoke("complete_task", {taskId});
    } catch (e) {
        toast.error("Error completing task", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function uncompleteTask(taskId: number) {
    try {
        return await invoke("uncomplete_task", {taskId});
    } catch (e) {
        toast.error("Error uncompleting task", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function deleteTask(taskId: number) {
    try {
        return await invoke("delete_task", {taskId});
    } catch (e) {
        toast.error("Error deleting task", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function setTyping(state: boolean) {
    try {
        return await invoke("set_typing", {state});
    } catch (e) {
        toast.error("Error setting typing state", {description: JSON.stringify(e)});
        throw e;
    }
}

export async function getTypingUsers() {
    try {
        return await invoke("get_typing_users");
    } catch (e) {
        toast.error("Error getting typing users", {
            description: JSON.stringify(e)
        });
        throw e;
    }
}

export async function version() {
    try {
        return await invoke("version");
    } catch (e) {
        toast.error("Error getting version", {description: JSON.stringify(e)});
    }
}

export async function checkUpdate() {
    try {
        return await invoke("check_update");
    } catch (e) {
        toast.error("Error checking for update", {description: JSON.stringify(e)});
    }
}
