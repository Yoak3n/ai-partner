/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

import type { MessageApi, DialogApi,NotificationApi, LoadingBarApi } from 'naive-ui'
declare global {
  interface Window {
    $loadingBar: LoadingBarApi
    $dialog: DialogApi
    $message: MessageApi
    $notification: NotificationApi
  }
  interface ImportMetaEnv {
    readonly __BUILD_TIME__: string;
  }
  
  interface ImportMeta {
    readonly env: ImportMetaEnv;
  }
}

