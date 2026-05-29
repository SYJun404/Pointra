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
     * 获取前端渲染所需的完整打包数据
     */
    static async getAllSettings() {
        // 使用 Promise.all 并行请求，如果内部独立调用了 fetchRawConfig，
        // 也可以优化为先获取一份 raw，再分别传参解析。
        const rustConfig = await this.fetchRawConfig();
        console.log(rustConfig);

        const general = GENERAL_SETTINGS_TEMPLATE.map((meta) => ({
            ...meta,
            enabled: rustConfig[meta.id],
        }));

        const shortcuts: ShortcutItem[] = SHORTCUT_LIST_TEMPLATE.map(
            (meta) => ({
                ...meta,
                keys: meta.rustKeys.map((key) => rustConfig[key]),
            }),
        );

        return { general, shortcuts };
    }

    /**
     * 更新所有设置项并同步到后端
     */
    static async updateAllSetting(
        shortcuts: ShortcutItem[],
        general: GeneralSetting[],
    ): Promise<boolean> {
        try {
            // 将 shortcuts 和 general 转换为 RustAppConfig 格式
            const config: RustAppConfig = {
                auto_hide: true,
                auto_play: false,
                hide_win_key: "Tab",
                pinned_key: "F1",
                point_key: "F3",
                search_win_code: "Digit2",
                search_win_modifiers: "SUPER",
                select_text_code: "Digit1",
                select_text_modifiers: "SUPER",
                theme: "light",
            };

            for (const setting of general) {
                Object.assign(config, { [setting.id]: setting.enabled });
            }
            for (const setting of shortcuts) {
                for (const key of setting.save) {
                    Object.assign(config, {
                        [key.id]: setting.keys[key.index],
                    });
                }
            }

            return await invoke<boolean>("update_config", {
                newConfig: config,
            });
        } catch (error) {
            console.error(error);
            return false;
        }
    }
}
