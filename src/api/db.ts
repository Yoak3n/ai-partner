import {invoke} from "@tauri-apps/api/core";
import { AppSetting } from "../types";
// 查询数据
export async function querySetting() {
    let setting: AppSetting = await invoke("get_config")
    console.log("get config: ", setting);
    
    return setting
}


// 更新数据
export async function updateAllSetting(app: AppSetting) {
    let r = await invoke("set_config", {newConfig: app})
    console.log("set config: ", r);
}