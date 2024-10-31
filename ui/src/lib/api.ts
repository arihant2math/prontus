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

export async function getCurrentChannelId(): Promise<any> {
    return await invoke("get_current_channel");
}

export async function setChannelMute(channelId: number, mute: boolean) {
    return await invoke("set_channel_mute", {channelId, mute});
}

export async function setChannelPin(channelId: number, pin: boolean) {
    return await invoke("set_channel_pin", {channelId, pin});
}

export async function setChannelAlias(channelId: number, alias?: string) {
    return await invoke("set_channel_alias", {channelId, alias});
}

export async function setChannelTitle(channelId: number, title: string) {
    return await invoke("set_channel_title", {channelId, title});
}

export async function setChannelCategory(channelId: number, categoryId: number) {
    return await invoke("set_channel_category", {channelId, categoryId});
}

export async function modifyChannelPermission(channelId: number, permission: string, value: string) {
    return await invoke("modify_channel_permission", {channelId, permission, value});
}

export async function deleteChannel(channelId: number) {
    return await invoke("delete_channel", {channelId});
}

export async function setChannelNotifications(channelId: number, level: string) {
    return await invoke("set_channel_notifications", {channelId, level});
}

export async function readChannel(channelId: number) {
    return await invoke("read_channel", {channelId});
}

export async function createDm(userId: number) {
    return await invoke("create_dm", {userId});
}

export async function userSearch(query: string) {
    return await invoke("user_search", {query});
}

export async function getAnnouncements() {
    return await invoke("get_announcements");
}

export async function getTasks() {
    return await invoke("get_tasks");
}

export async function completeTask(taskId: number) {
    return await invoke("complete_task", {taskId});
}

export async function uncompleteTask(taskId: number) {
    return await invoke("uncomplete_task", {taskId});
}

export async function deleteTask(taskId: number) {
    return await invoke("delete_task", {taskId});
}

export async function setTyping(state: boolean) {
    return await invoke("set_typing", {state});
}

export async function getTypingUsers() {
    return await invoke("get_typing_users");
}
