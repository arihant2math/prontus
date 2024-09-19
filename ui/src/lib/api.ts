import {invoke} from "@tauri-apps/api/core";

export async function getCode(email: string) {
    return await invoke("get_code", {email});
}

export async function sendCode(email: string, code) {
    return await invoke("send_code", {email, code})
}

export async function load() {
    return await invoke("load");
}

export async function getCurrentUser() {
    return await invoke("get_current_user");
}

export async function getUser(id: number) {
    return await invoke("get_user", {id});
}

export async function loadChannel(id: number) {
    await invoke("load_channel", {id});
}

export async function getChannelList() {
    return await invoke("get_channel_list");
}

export async function getChannelInfo() {
    return await invoke("get_channel_info");
}

export async function getMessage(id: number) {
    return await invoke("get_message", {id});
}

export async function getMessages(): Promise<object[]> {
    return await invoke("get_messages");
}

export async function getMoreMessages(lastMessageId: number): Promise<object[]> {
    return await invoke("get_more_messages", {lastMessageId});
}

export async function getParentMessages() {
    return await invoke("get_parent_messages");
}

export async function loadMessages() {
    return await invoke("load_messages");
}

export async function sendMessage(message: string, thread?: number) {
    return await invoke("send_message", {message, thread});
}

export async function deleteMessage(messageId: number) {
    return await invoke("delete_message", {messageId});
}

export async function setReactionState(messageId: number, reactionId: number, active: boolean) {
    return await invoke("set_reaction_state", {messageId, reactionId, active});
}

export async function getChannelUsers(id: number) {
    return await invoke("get_channel_users", {id})
}

export async function loadChannelUsers(id: number) {
    return await invoke("load_channel_users", {id})
}

export async function getSettings() {
    return await invoke("get_settings");
}

export async function setSettings(settings) {
    return await invoke("set_settings", {settings});
}

export async function getCurrentChannelId(): Promise<number> {
    return await invoke("get_current_channel_id");
}