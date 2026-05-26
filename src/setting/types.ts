/** 一条快捷键配置 */
export interface ShortcutItem {
    id: string;
    label: string;
    keys: string[];
    defaultKeys: string[];
}

/** 通用设置项 */
export interface GeneralSetting {
    id: string;
    label: string;
    description: string;
    enabled: boolean;
}
