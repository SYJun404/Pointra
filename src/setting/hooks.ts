import { useState, useEffect, useRef, useCallback } from "react";
import type { ShortcutItem, GeneralSetting } from "./types";
import { GENERAL_SETTINGS as DEFAULT_GENERAL, SHORTCUT_LIST } from "./data";
import { eventToKeys, getConflictIds } from "./utils";

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
    const [shortcuts, setShortcuts] = useState<ShortcutItem[]>(SHORTCUT_LIST);
    const [generalSettings, setGeneralSettings] =
        useState<GeneralSetting[]>(DEFAULT_GENERAL);
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

        const hasModifier = e.ctrlKey || e.metaKey || e.altKey || e.shiftKey;
        const isFunctionKey = e.key.startsWith("F") && e.key.length <= 3;
        const isTab = e.key === "Tab";

        if (!hasModifier && !isFunctionKey && !isTab) {
            showToast(
                "请至少包含一个修饰键（Ctrl / ⌘ / Alt / Shift）或使用功能键",
            );
            return;
        }

        const newKeys = eventToKeys(e);
        if (newKeys.length === 0) return;

        const conflict = shortcuts.find(
            (s) =>
                s.id !== id &&
                s.keys.length === newKeys.length &&
                s.keys.every((k, i) => k === newKeys[i]),
        );

        setShortcuts((prev) =>
            prev.map((s) => (s.id === id ? { ...s, keys: newKeys } : s)),
        );
        setRecordingId(null);
        setHasChanges(true);

        if (conflict) {
            showToast(`⚠️ 该快捷键与「${conflict.label}」冲突`);
        } else {
            showToast("✅ 快捷键已更新");
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
