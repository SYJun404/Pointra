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

export function useOnWindowChange(onShow: () => void, onHide: () => void) {
    const savedOnShow = useRef(onShow);
    const savedOnHide = useRef(onHide);

    useEffect(() => {
        savedOnShow.current = onShow;
    }, [onShow]);

    useEffect(() => {
        savedOnHide.current = onHide;
    }, [onHide]);

    useEffect(() => {
        async function setupListener() {
            const appWindow = getCurrentWindow();

            // 监听窗口显示/获得焦点事件
            await appWindow.listen("tauri://focus", () => {
                savedOnShow.current();
            });

            // 监听窗口隐藏/失焦事件
            await appWindow.listen("tauri://blur", () => {
                savedOnHide.current();
            });
        }

        setupListener();
    }, []);
}
