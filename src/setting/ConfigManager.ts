// ConfigManager.ts
import { invoke } from "@tauri-apps/api/core";
import { RustAppConfig, GeneralSetting, ShortcutItem } from "./types";
import {
    GENERAL_SETTINGS_TEMPLATE,
    SHORTCUT_LIST_TEMPLATE,
} from "./settingsMeta.ts";

export class ConfigManager {
    /**
     * 从后端获取原始的 RustAppConfig
     */
    static async fetchRawConfig(): Promise<RustAppConfig> {
        return await invoke<RustAppConfig>("get_config");
    }

    /**
     * 获取转换后的通用设置数组
     */
    static async getGeneralSettings(): Promise<GeneralSetting[]> {
        const rustConfig = await this.fetchRawConfig();
        return GENERAL_SETTINGS_TEMPLATE.map((meta) => ({
            ...meta,
            enabled: rustConfig[meta.id],
        }));
    }

    /**
     * 获取转换后的快捷键设置数组
     */
    static async getShortcutList(): Promise<ShortcutItem[]> {
        const rustConfig = await this.fetchRawConfig();
        return SHORTCUT_LIST_TEMPLATE.map((meta) => ({
            ...meta,
            keys: meta.keys.map((key) => rustConfig[key]),
        }));
    }

    /**
     * 获取前端渲染所需的完整打包数据
     */
    static async getAllSettings() {
        // 使用 Promise.all 并行请求，如果内部独立调用了 fetchRawConfig，
        // 也可以优化为先获取一份 raw，再分别传参解析。
        const rustConfig = await this.fetchRawConfig();

        const general = GENERAL_SETTINGS_TEMPLATE.map((meta) => ({
            ...meta,
            enabled: rustConfig[meta.id],
        }));

        const shortcuts: ShortcutItem[] = [
            {
                id: "point_key",
                label: "定点快捷键",
                keys: [rustConfig.point_key],
                defaultKeys: ["F3"],
            },
            {
                id: "pinned_key",
                label: "置顶快捷键",
                keys: [rustConfig.pinned_key],
                defaultKeys: ["F1"],
            },
            {
                id: "hide_win_key",
                label: "隐藏窗口",
                keys: [rustConfig.hide_win_key],
                defaultKeys: ["Tab"],
            },
            {
                id: "select_text",
                label: "划词快捷键",
                keys: [
                    rustConfig.select_text_modifiers,
                    rustConfig.select_text_code,
                ],
                defaultKeys: ["SUPER", "Digit1"],
            },
            {
                id: "search_win",
                label: "打开搜索",
                keys: [
                    rustConfig.search_win_modifiers,
                    rustConfig.search_win_code,
                ],
                defaultKeys: ["SUPER", "Digit2"],
            },
        ];

        return { general, shortcuts };
    }

    /**
     * 更新单个通用设置项并同步到后端
     */
    static async updateGeneralSetting(
        id: keyof Pick<RustAppConfig, "auto_hide" | "auto_play">,
        value: boolean,
    ) {
        const currentConfig = await this.fetchRawConfig();
        currentConfig[id] = value;
        await invoke("update_config", { newConfig: currentConfig });
    }
}
