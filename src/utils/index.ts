import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
export * from './router'
export * from './theme'
export const switchDialogWindow= async() => {
  let oldWindow = await WebviewWindow.getByLabel("dialog");
  if (oldWindow) {
      if (await oldWindow.isVisible()){
        oldWindow.hide();
      }else{
        oldWindow.show();
      }
      return;
  }else{
    const webviewWindow = new WebviewWindow("dialog", {
      url: "/dialog",
      title: "对话",
      width: 400,
      height: 300,
      center: true,
      resizable: false,
      alwaysOnTop: true,
      decorations: false,
      transparent: true,
    });
    webviewWindow.once("tauri://created", () => {
        console.log("对话窗口已创建");
    });
    webviewWindow.once("tauri://error", (e) => {
        console.log(e);
    })
  }
}
export const createDialogWindow= async() => {
    let oldWindow = await WebviewWindow.getByLabel("dialog");
    if (oldWindow) {
        oldWindow.show();
        oldWindow.setAlwaysOnTop(true);
        return;
    }
    const webviewWindow = new WebviewWindow("dialog", {
        url: "/dialog",
        title: "对话",
        width: 400,
        height: 300,
        center: true,
        resizable: false,
        alwaysOnTop: true,
    });
    webviewWindow.once("tauri://created", () => {
        console.log("对话窗口已创建");
    });
    webviewWindow.once("tauri://error", (e) => {
        console.log(e);
    })
}
export const closeDialogWindow= async()=>{
    const oldWindow = await WebviewWindow.getByLabel("setting");
    if (oldWindow) {
        oldWindow.close();
    }
}



type DebounceFunction<T extends any[]> = (...args: T) => void;

export function debounce<T extends any[]>(fn: DebounceFunction<T>, delay: number): DebounceFunction<T> {
  let timer: ReturnType<typeof setTimeout> | null = null;

  return (...args: T) => {
    if (timer) {
      clearTimeout(timer);
    }
    timer = setTimeout(() => {
      fn(...args);
    }, delay);
  };
}

export function throttle<T extends (...args: any[]) => void>(
    func: T,
    wait: number
  ): (...args: Parameters<T>) => void {
  let lastExecTime = 0; // 上次执行的时间戳
  let timeoutId: ReturnType<typeof setTimeout> | null = null;

  return (...args: Parameters<T>) => {
    const now = Date.now();
    const timeSinceLastExec = now - lastExecTime;

    // 如果距离上次执行的时间超过 wait，立即执行
    if (timeSinceLastExec >= wait) {
      if (timeoutId) {
        clearTimeout(timeoutId);
        timeoutId = null;
      }
      lastExecTime = now;
      
      func(...args);
    } else if (!timeoutId) {
      // 否则，设置一个定时器，在剩余时间后执行
      timeoutId = setTimeout(() => {
        lastExecTime = Date.now();
        func(...args);
        timeoutId = null;
      }, wait - timeSinceLastExec);
    }
  };
}

export const addCopyButtons = ()=>{
  const codeBlocks = document.querySelectorAll<HTMLElement>('pre code');
  if (!codeBlocks.length) return;

  codeBlocks.forEach((codeBlock) => {
      // 创建复制按钮
      if (codeBlock && !codeBlock.querySelector('.copy-button')){
          const copyButton = document.createElement('button');
          copyButton.className = 'copy-button';
          copyButton.textContent = '复制代码';
     
          // 将按钮插入到代码块容器中
          const pre = codeBlock.parentElement;
          if (!pre) return;
          pre.style.position = 'relative';
          pre.appendChild(copyButton);
          
          // 添加点击事件
          copyButton.addEventListener('click', () => {
              navigator.clipboard.writeText(codeBlock.innerText).then(() => {
                  copyButton.textContent = '已复制';
                  setTimeout(() => (copyButton.textContent = '复制代码'), 1500);
              }).catch(err => window.$message.error('复制失败: '+ err));
          });
      }
     
  });
  
}