import {defineStore} from 'pinia'
export type CloseAction = 'hide' | 'exit' | 'ask'

export const useAppStore = defineStore('app', 
    {
    state: () => ({
        generating: false,
        closeAction: localStorage.getItem('closeAction') as CloseAction || 'ask'
    }),
    actions: {
        setGenerating(generating: boolean) {
            this.generating = generating
        },
        setCloseAction(action: CloseAction) {
            this.closeAction = action
            localStorage.setItem('closeAction', action)
        }
    }
})