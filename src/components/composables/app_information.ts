export interface AppInfo {
    name: string,
    version: string,
    tauriVersion: string,
    buildNumber: string,
    buildDate: string,
    configPath: string,
    logo: string,
}

import { getVersion, getName, getTauriVersion } from '@tauri-apps/api/app'
import versionJson from '../../utils/versionJson.json'
export async function getAppInfo ():Promise<AppInfo> {
    let appInfo: AppInfo = {
        name: await getName(),
        tauriVersion: await getTauriVersion(),
        version: await getVersion(),
        buildNumber: "Unknown",
        buildDate: new Date(versionJson.compileTime).toLocaleDateString(),
        configPath: "Unknown",
        logo: "Unknown",
    }
    return appInfo;
}