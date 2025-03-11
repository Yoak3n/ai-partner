import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import type{ AppSetting } from "../../types";
export const useApiStore = defineStore("api", {
  state: () => ({
    api: {
        url: "",
        key: "",
        model:""
    },
    hotkey:{
        dialog:""
    },
    modelHistory: [] as string[]
  }),
  actions: {
        async getApifromConfig (){
          const setting:AppSetting = await invoke("get_config")
          this.api.url = setting.api.url
          this.api.key = setting.api.key
          this.api.model = setting.api.model
          this.hotkey.dialog = setting.hotkey.dialog
      },
      initModelHistory() {
        const history = localStorage.getItem('modelHistory')
        if (history) {
            this.modelHistory = JSON.parse(history)
        }
      },
      addModelToHistory(model: string) {
        if (!this.modelHistory.includes(model)) {
            this.modelHistory.push(model)
            localStorage.setItem('modelHistory', JSON.stringify(this.modelHistory))
          }
      },
      clearModelHistory() {
        this.modelHistory = []
        localStorage.removeItem('modelHistory')
      }
  }
})