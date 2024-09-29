import {invoke} from "@tauri-apps/api/core";

export async function getCode(email: string): Promise<void> {
    return await invoke("get_code", {email});
}

export async function sendCode(email: string, code: string): Promise<void> {
    return await invoke("send_code", {email, code})
}

export async function load(): Promise<void> {
    return await invoke("load");
}

export async function getCurrentUser() {
    return await invoke("get_current_user");
}

export async function getUser(id: number) {
    return await invoke("get_user", {id});
}

export async function loadChannel(id: number): Promise<void> {
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

export async function getMessages(): Promise<any[]> {
    return await invoke("get_messages");
}

export async function getMoreMessages(lastMessageId: number): Promise<any[]> {
    return await invoke("get_more_messages", {lastMessageId});
}

export async function getParentMessages(): Promise<any[]> {
    return await invoke("get_parent_messages");
}

export async function loadMessages(): Promise<void> {
    return await invoke("load_messages");
}

export async function editMessage(messageId: number, message: String) {
    return await invoke("edit_message", {messageId, message});
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

export async function getChannelUsers(id: number): Promise<any> {
    return await invoke("get_channel_users", {id})
}

export async function loadChannelUsers(id: number): Promise<void> {
    return await invoke("load_channel_users", {id})
}

export async function getSettings() {
    return await invoke("get_settings");
}

export async function setSettings(settings: any): Promise<void> {
    return await invoke("set_settings", {settings});
}

export async function getCurrentChannelId(): Promise<number> {
    return await invoke("get_current_channel_id");
}

export async function setChannelMute(channelId: number, mute: boolean) {
    return await invoke("set_channel_mute", {channelId, mute});
}
