import { useState, useEffect, useRef, useCallback } from "react";
import type { ShortcutItem, GeneralSetting } from "./types";
import { eventToKeys, getConflictIds, normalizeKeys } from "./utils";
import { ConfigManager } from "./ConfigManager.ts";

export interface UseShortcutManagerReturn {
    shortcuts: ShortcutItem[];
    generalSettings: GeneralSetting[];
    hasChanges: boolean;
    recordingId: string | null;
    recordingRef: React.RefObject<HTMLDivElement | null>;
    conflictIds: Set<string>;
    toastMsg: string | null;
    showToast: (msg: string) => void;
    toggleGeneral: (id: string) => void;
    startRecording: (id: string) => void;
    cancelRecording: () => void;
    handleKeyDown: (e: React.KeyboardEvent, id: string) => void;
    resetToDefault: (id: string) => void;
    resetAllShortcuts: () => void;
    handleSave: () => void;
}

export function useShortcutManager(): UseShortcutManagerReturn {
    const [shortcuts, setShortcuts] = useState<ShortcutItem[]>([]);
    const [generalSettings, setGeneralSettings] = useState<GeneralSetting[]>(
        [],
    );
    const [recordingId, setRecordingId] = useState<string | null>(null);
    const [toastMsg, setToastMsg] = useState<string | null>(null);
    const [hasChanges, setHasChanges] = useState(false);
    const recordingRef = useRef<HTMLDivElement>(null);

    /* ---- Toast ---- */
    const showToast = useCallback((msg: string) => {
        setToastMsg(msg);
        setTimeout(() => setToastMsg(null), 2000);
    }, []);

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

        const keys = eventToKeys(e); // 建议内部用 e.code + modifier

        if (!keys || keys.length === 0) return;

        const hasModifier = e.ctrlKey || e.metaKey || e.altKey || e.shiftKey;

        const isFunctionKey =
            e.code.startsWith("F") && /^F\d{1,2}$/.test(e.code);

        const isAllowedSingle =
            isFunctionKey || e.code === "Tab" || e.code === "Escape";

        // 限制规则：普通键必须带修饰键
        if (!hasModifier && !isAllowedSingle) {
            showToast("请至少包含 Ctrl / ⌘ / Alt / Shift 或使用功能键");
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
            showToast(`该快捷键与「${conflict.label}」冲突`);
        } else {
            showToast("快捷键已更新");
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

    const resetAllShortcuts = () => {
        setShortcuts((prev) =>
            prev.map((s) => ({ ...s, keys: [...s.defaultKeys] })),
        );
        setHasChanges(true);
        showToast("所有快捷键已恢复默认");
    };

    const handleSave = () => {
        setHasChanges(false);
        showToast("✅ 设置已保存");
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
        toastMsg,
        showToast,
        toggleGeneral,
        startRecording,
        cancelRecording,
        handleKeyDown,
        resetToDefault,
        resetAllShortcuts,
        handleSave,
    };
}
