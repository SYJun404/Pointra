import type { ShortcutItem, GeneralSetting } from "./types";

export const GENERAL_SETTINGS: GeneralSetting[] = [
    {
        id: "auto-hide",
        label: "鼠标离开后自动隐藏",
        description: "窗口失去焦点后自动隐藏到后台",
        enabled: true,
    },
    {
        id: "hover-trigger",
        label: "鼠标悬停取词",
        description: "鼠标悬停在单词上时自动查询",
        enabled: false,
    },
    {
        id: "pin-on-startup",
        label: "启动时固定窗口",
        description: "应用启动后窗口默认置顶显示",
        enabled: false,
    },
    {
        id: "smooth-translate",
        label: "平滑翻译动画",
        description: "翻译结果展示使用过渡动画效果",
        enabled: true,
    },
];

export const SHORTCUT_LIST: ShortcutItem[] = [
    {
        id: "search",
        label: "打开搜索",
        keys: ["Ctrl", "Shift", "S"],
        defaultKeys: ["Ctrl", "Shift", "S"],
    },
    {
        id: "translate-clipboard",
        label: "翻译剪贴板内容",
        keys: ["Ctrl", "Shift", "D"],
        defaultKeys: ["Ctrl", "Shift", "D"],
    },
    {
        id: "toggle-pin",
        label: "切换窗口固定",
        keys: ["F1"],
        defaultKeys: ["F1"],
    },
    {
        id: "hide-window",
        label: "隐藏窗口",
        keys: ["Tab"],
        defaultKeys: ["Tab"],
    },
    {
        id: "voice-input",
        label: "语音输入",
        keys: ["Ctrl", "Shift", "V"],
        defaultKeys: ["Ctrl", "Shift", "V"],
    },
    {
        id: "copy-result",
        label: "复制翻译结果",
        keys: ["Ctrl", "C"],
        defaultKeys: ["Ctrl", "C"],
    },
    {
        id: "open-setting",
        label: "打开设置",
        keys: ["Ctrl", ","],
        defaultKeys: ["Ctrl", ","],
    },
];

export const MODIFIER_MAP: Record<string, string> = {
    Control: "Ctrl",
    Meta: "⌘",
    Alt: "Alt",
    Shift: "Shift",
};
