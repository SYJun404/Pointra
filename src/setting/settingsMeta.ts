import { RustAppConfig, ShortcutItem } from "./types";

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
    rustKeys: Keys[];
    defaultKeys: string[];
    save: {
        id: string;
        index: number;
    }[];
}[] = [
    {
        id: "point_key",
        label: "点位快捷键 (示例)",
        rustKeys: ["point_key"],
        defaultKeys: ["F3"],
        save: [
            {
                id: "point_key",
                index: 0,
            },
        ],
    },
    {
        id: "pinned_key",
        label: "置顶快捷键",
        rustKeys: ["pinned_key"],
        defaultKeys: ["F1"],
        save: [
            {
                id: "pinned_key",
                index: 0,
            },
        ],
    },
    {
        id: "hide_win_key",
        label: "隐藏窗口快捷键",
        rustKeys: ["hide_win_key"],
        defaultKeys: ["Tab"],
        save: [
            {
                id: "hide_win_key",
                index: 0,
            },
        ],
    },
    {
        id: "select_text",
        label: "划词快捷键",
        rustKeys: ["select_text_modifiers", "select_text_code"],
        defaultKeys: ["SUPER", "Digit1"],
        save: [
            {
                id: "select_text_modifiers",
                index: 0,
            },
            {
                id: "select_text_code",
                index: 1,
            },
        ],
    },
    {
        id: "search_win",
        label: "打开搜索",
        rustKeys: ["search_win_modifiers", "search_win_code"],
        defaultKeys: ["SUPER", "Digit2"],
        save: [
            {
                id: "search_win_modifiers",
                index: 0,
            },
            {
                id: "search_win_code",
                index: 1,
            },
        ],
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
