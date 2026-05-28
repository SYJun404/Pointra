import { RustAppConfig } from "./types";

type Keys = keyof Pick<
    RustAppConfig,
    | "select_text_modifiers"
    | "select_text_code"
    | "search_win_modifiers"
    | "search_win_code"
    | "point_key"
    | "pinned_key"
    | "hide_win_key"
>;

export const GENERAL_SETTINGS_TEMPLATE: {
    id: keyof Pick<RustAppConfig, "auto_hide" | "auto_play">;
    label: string;
    description: string;
}[] = [
    {
        id: "auto_hide" as const,
        label: "自动隐藏",
        description: "鼠标离开窗口后自动隐藏界面",
    },
    {
        id: "auto_play" as const,
        label: "自动播放",
        description: "查词成功后自动播放单词音频",
    },
];

export const SHORTCUT_LIST_TEMPLATE: {
    id: string;
    label: string;
    keys: Keys[];
    defaultKeys: string[];
}[] = [
    {
        id: "point_key",
        label: "点位快捷键 (示例)",
        keys: ["point_key"],
        defaultKeys: ["F3"],
    },
    {
        id: "pinned_key",
        label: "置顶快捷键",
        keys: ["pinned_key"],
        defaultKeys: ["F1"],
    },
    {
        id: "hide_win_key",
        label: "隐藏窗口快捷键",
        keys: ["hide_win_key"],
        defaultKeys: ["Tab"],
    },
    {
        id: "select_text",
        label: "划词快捷键",
        keys: ["select_text_modifiers", "select_text_code"],
        defaultKeys: ["SUPER", "Digit1"],
    },
    {
        id: "search_win",
        label: "打开搜索",
        keys: ["search_win_modifiers", "search_win_code"],
        defaultKeys: ["SUPER", "Digit2"],
    },
];

export const MODIFIER_MAP: Record<string, string> = {
    Control: "Ctrl",
    SUPER: "Cmd",
    Alt: "Alt",
    Shift: "Shift",
    Digit1: "1",
    Digit2: "2",
    Digit3: "3",
    Digit4: "4",
    Digit5: "5",
    Digit6: "6",
    Digit7: "7",
    Digit8: "8",
    Digit9: "9",
    Digit0: "0",
};
