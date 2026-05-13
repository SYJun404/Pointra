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

export function useOnWindowChange(onHide: () => void) {
    const savedOnHide = useRef(onHide);

    useEffect(() => {
        savedOnHide.current = onHide;
    }, [onHide]);

    useEffect(() => {
        async function setupListener() {
            const appWindow = getCurrentWindow();

            // 监听窗口隐藏/失焦事件
            await appWindow.listen("tauri://blur", () => {
                // 判断窗口是否为隐藏状态
                appWindow.isVisible().then((isShow) => {
                    if (!isShow) {
                        savedOnHide.current();
                    }
                });
            });
        }

        setupListener();
    }, []);
}
