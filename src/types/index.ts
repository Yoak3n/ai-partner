export interface MessageItem {
    role: string;
    content?: string;
    reasoning_content?:string;
    timestamp?: number;
}

export interface ErrorMessage {
    id:number;
    message: string;
}

export interface AppSetting  {
    api: API;
    hotkey: Hotkey;
}
export interface API {
    url: string;
    key: string;
    model: string;
}
export interface Hotkey{
    dialog:string
}

export interface Segment {
    raw:string;
    index?: number;
    hash: number;
    html?:string
}
export * from './listener'