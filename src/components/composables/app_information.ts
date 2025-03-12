export interface AppInfo {
    name: string,
    version: string,
    tauriVersion: string,
    buildNumber: string,
    buildDate: string,
    configPath: string,
    logo: string,
}

export interface VersionComparation {
    current_version: string,
    version: string,
}

export type DownloadEvent =
  | {
      event: 'Started';
      data: {
        contentLength: number;
      };
    }
  | {
      event: 'Progress';
      data: {
        chunkLength: number;
      };
    }
  | {
      event: 'Finished';
      data: {
        downloadId: number;
      };
    };

import { getVersion, getName, getTauriVersion } from '@tauri-apps/api/app'
export async function getAppInfo ():Promise<AppInfo> {
    const compileTime = import.meta.env.__BUILD_TIME__;
    let appInfo: AppInfo = {
        name: await getName(),
        tauriVersion: await getTauriVersion(),
        version: await getVersion(),
        buildNumber: "Unknown",
        buildDate: new Date(compileTime).toLocaleDateString(),
        configPath: "Unknown",
        logo: "Unknown",
    }
    return appInfo;
}