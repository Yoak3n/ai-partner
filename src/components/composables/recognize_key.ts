type OSType = 'Darwin' | 'Windows' | 'Linux' | string;
let osType: OSType = 'Windows';
const keyMap:Record<string, string> = {
    Backquote: '`',
    Backslash: '\\',
    BracketLeft: '[',
    BracketRight: ']',
    Comma: ',',
    Equal: '=',
    Minus: '-',
    Plus: 'PLUS',
    Period: '.',
    Quote: "'",
    Semicolon: ';',
    Slash: '/',
    Backspace: 'Backspace',
    CapsLock: 'Capslock',
    ContextMenu: 'Contextmenu',
    Space: 'Space',
    Tab: 'Tab',
    Convert: 'Convert',
    Delete: 'Delete',
    End: 'End',
    Help: 'Help',
    Home: 'Home',
    PageDown: 'Pagedown',
    PageUp: 'Pageup',
    Escape: 'Esc',
    PrintScreen: 'Printscreen',
    ScrollLock: 'Scrolllock',
    Pause: 'Pause',
    Insert: 'Insert',
    Suspend: 'Suspend',
};

export function keyDown(e:KeyboardEvent, setKey:(value: string) => void) {
    e.preventDefault();
    if (e.key === 'Backspace') {
        setKey('');
    } else {
        let newValue = '';
        if (e.ctrlKey) {
            newValue = 'ctrl';
        }
        if (e.shiftKey) {
            newValue = `${newValue}${newValue.length > 0 ? '+' : ''}shift`;
        }
        if (e.metaKey) {
            newValue = `${newValue}${newValue.length > 0 ? '+' : ''}${osType === 'Darwin' ? 'Command' : 'Super'}`;
        }
        if (e.altKey) {
            newValue = `${newValue}${newValue.length > 0 ? '+' : ''}alt`;
        }
        let code = e.code;
        if (code.startsWith('Key')) {
            code = code.substring(3);
        } else if (code.startsWith('Digit')) {
            code = code.substring(5);
        } else if (code.startsWith('Numpad')) {
            code = 'Num' + code.substring(6);
        } else if (code.startsWith('Arrow')) {
            code = code.substring(5);
        } else if (code.startsWith('Intl')) {
            code = code.substring(4);
        } else if (/F\d+/.test(code)) {
        } else if (keyMap[code] !== undefined) {
            code = keyMap[code];
        } else {
            code = '';
        }
        setKey(`${newValue}${newValue.length > 0 && code.length > 0 ? '+' : ''}${code}`);
    }
}