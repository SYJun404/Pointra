import { useEffect, useRef } from "react";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function useWindowShortcut() {
    useEffect(() => {
        const handleKeyDown = async (e: KeyboardEvent) => {
            if (e.key === "Escape") {
                await getCurrentWebviewWindow().hide();
            }
        };
        window.addEventListener("keydown", handleKeyDown);
        return () => window.removeEventListener("keydown", handleKeyDown);
    }, []);
}

export function useOnWindowShow(callback: () => void) {
    const savedCallback = useRef(callback);

    useEffect(() => {
        savedCallback.current = callback;
    }, [callback]);

    useEffect(() => {
        let unlisten: (() => void) | undefined;

        async function setupListener() {
            const appWindow = getCurrentWindow();
            const unlistenFunc = await appWindow.listen("tauri://focus", () => {
                savedCallback.current();
            });

            unlisten = unlistenFunc;
        }

        setupListener();

        // 组件卸载时取消监听，防止内存泄漏
        return () => {
            if (unlisten) unlisten();
        };
    }, []);
}
