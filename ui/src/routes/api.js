import {invoke} from "@tauri-apps/api/core";

export async function getCode(email) {
    return await invoke("get_code", {email});
}

export async function sendCode(email, code) {
    return await invoke("send_code", {email, code})
}

export async function load() {
    return await invoke("load");
}

export async function loadChannel(id) {
    await invoke("load_channel", {id});
}

export async function getChannelList() {
    return await invoke("get_channel_list");
}

export async function getMessages() {
    return await invoke("get_messages");
}

export async function getMoreMessages(lastMessageId) {
    return await invoke("get_more_messages", {lastMessageId});
}

export async function loadMessages() {
    return await invoke("load_messages");
}

export async function sendMessage(message) {
    return await invoke("send_message", {message});
}