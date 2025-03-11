// 使用Web Worker进行解析
let worker:Worker|null = null
export const parseInWorker = (raw:string):Promise<string> => {
  return new Promise((resolve) => {
    if (!worker) {
      worker = new Worker('./markdown.worker.js')
    }
    worker.onmessage = (e) => resolve(e.data)
    worker.postMessage(raw)
  })
}

// 简单哈希函数
export const simpleHash = (str:string) => {
  let hash = 0
  for (let i = 0; i < str.length; i++) {
    hash = ((hash << 5) - hash) + str.charCodeAt(i)
    hash |= 0
  }
  return hash
}
