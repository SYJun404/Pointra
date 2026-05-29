import { useState, useEffect, useRef, useCallback } from "react";
import type { ShortcutItem, GeneralSetting } from "./types";
import { eventToKeys, getConflictIds, normalizeKeys } from "./utils";
import { ConfigManager } from "./ConfigManager.ts";
import { toast, ToastContentValue } from "@heroui/react";
import { usePlatform } from "../hooks/usePlatform.ts";
import { getCurrentWindow } from "@tauri-apps/api/window";

export interface UseShortcutManagerReturn {
    shortcuts: ShortcutItem[];
    generalSettings: GeneralSetting[];
    hasChanges: boolean;
    recordingId: string | null;
    recordingRef: React.RefObject<HTMLDivElement | null>;
    conflictIds: Set<string>;
    showToast: (msg: string) => void;
    toggleGeneral: (id: string) => void;
    startRecording: (id: string) => void;
    cancelRecording: () => void;
    handleKeyDown: (e: React.KeyboardEvent, id: string) => void;
    resetToDefault: (id: string) => void;
    handleSave: () => void;
    closeApp: () => void;
}

export function useShortcutManager(): UseShortcutManagerReturn {
    const [shortcuts, setShortcuts] = useState<ShortcutItem[]>([]);
    const [generalSettings, setGeneralSettings] = useState<GeneralSetting[]>(
        [],
    );
    const [recordingId, setRecordingId] = useState<string | null>(null);
    const [hasChanges, setHasChanges] = useState(false);
    const [hasSaved, setHasSaved] = useState(false);
    const recordingRef = useRef<HTMLDivElement>(null);
    const { isMac } = usePlatform();
    const initConfig = useRef<null | {
        shortcuts: ShortcutItem[];
        generalSettings: GeneralSetting[];
    }>(null);

    /* ---- Toast ---- */
    const showToast = useCallback(
        (
            msg: string,
            variant?: ToastContentValue["variant"],
            timeout?: number,
        ) => {
            toast(msg, {
                timeout: timeout ?? 2000,
                variant: variant ?? "success",
            });
        },
        [],
    );

    /* ---- 通用设置 ---- */
    const toggleGeneral = (id: string) => {
        setGeneralSettings((prev) =>
            prev.map((s) => (s.id === id ? { ...s, enabled: !s.enabled } : s)),
        );
        setHasChanges(true);
    };

    /* ---- 快捷键录制 ---- */
    const startRecording = (id: string) => {
        setRecordingId(id);
    };

    const cancelRecording = () => {
        setRecordingId(null);
    };

    const handleKeyDown = (e: React.KeyboardEvent, id: string) => {
        e.preventDefault();
        e.stopPropagation();

        // 防止长按重复触发
        if (e.repeat) return;

        const isModifierOnly =
            e.key === "Control" ||
            e.key === "Shift" ||
            e.key === "Alt" ||
            e.key === "Meta";

        // 只按修饰键不记录
        if (isModifierOnly) return;

        const keys = eventToKeys(e, isMac); // 建议内部用 e.code + modifier
        console.log(keys);

        if (!keys || keys.length === 0) return;

        const hasModifier = e.ctrlKey || e.metaKey || e.altKey || e.shiftKey;

        const isFunctionKey =
            e.code.startsWith("F") && /^F\d{1,2}$/.test(e.code);

        const isAllowedSingle =
            isFunctionKey || e.code === "Tab" || e.code === "Escape";

        // 限制规则：普通键必须带修饰键
        if (!hasModifier && !isAllowedSingle) {
            showToast(
                "请至少包含 Ctrl / ⌘ / Alt / Shift 或使用功能键",
                "warning",
                4000,
            );
            return;
        }

        // 标准化 key（避免顺序问题）
        const normalized = normalizeKeys(keys);

        const conflict = shortcuts.find((s) => {
            if (s.id === id) return false;

            const a = normalizeKeys(s.keys);

            return (
                a.length === normalized.length &&
                a.every((k) => normalized.includes(k))
            );
        });

        setShortcuts((prev) =>
            prev.map((s) => (s.id === id ? { ...s, keys: normalized } : s)),
        );

        setRecordingId(null);
        setHasChanges(true);

        if (conflict) {
            showToast(`该快捷键与「${conflict.label}」冲突`, "danger");
        }
    };

    const resetToDefault = (id: string) => {
        setShortcuts((prev) =>
            prev.map((s) =>
                s.id === id ? { ...s, keys: [...s.defaultKeys] } : s,
            ),
        );
        setHasChanges(true);
        showToast("已恢复默认");
    };

    const handleSave = async () => {
        setHasChanges(false);
        const success = await ConfigManager.updateAllSetting(
            shortcuts,
            generalSettings,
        );

        if (success) {
            setHasSaved(true);
            showToast("保存成功, 2秒后自动关闭", "success");
            setTimeout(() => {
                getCurrentWindow().close();
            }, 2000);
        } else {
            showToast("保存失败, 已恢复初始状态", "danger", 3000);
            if (initConfig.current) {
                setShortcuts(initConfig.current.shortcuts);
                setGeneralSettings(initConfig.current.generalSettings);
            }
        }
    };

    const closeApp = async () => {
        if (!hasSaved) {
            await ConfigManager.updateAllSetting(shortcuts, generalSettings);
        }
        getCurrentWindow().close();
    };

    /* 点击外部停止录制 */
    useEffect(() => {
        if (recordingId === null) return;
        const handler = (e: MouseEvent) => {
            if (
                recordingRef.current &&
                !recordingRef.current.contains(e.target as Node)
            ) {
                setRecordingId(null);
            }
        };
        const timer = setTimeout(() => {
            document.addEventListener("click", handler);
        }, 0);
        return () => {
            clearTimeout(timer);
            document.removeEventListener("click", handler);
        };
    }, [recordingId]);

    // 初始化加载配置
    useEffect(() => {
        ConfigManager.getAllSettings()
            .then(({ general, shortcuts }) => {
                setGeneralSettings(general);
                setShortcuts(shortcuts);
                initConfig.current = { shortcuts, generalSettings };
            })
            .catch((err) => console.error("加载配置失败:", err));
    }, []);

    const conflictIds = getConflictIds(shortcuts);

    return {
        shortcuts,
        generalSettings,
        hasChanges,
        recordingId,
        recordingRef,
        conflictIds,
        showToast,
        toggleGeneral,
        startRecording,
        cancelRecording,
        handleKeyDown,
        resetToDefault,
        handleSave,
        closeApp,
    };
}
