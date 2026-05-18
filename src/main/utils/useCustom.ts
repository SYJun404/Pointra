import { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function useWindowListener(isPinned: boolean) {
    const isPinnedRef = useRef(isPinned);

    useEffect(() => {
        isPinnedRef.current = isPinned;
    }, [isPinned]);

    useEffect(() => {
        const handleMouseEnter = () => {
            invoke("update_hover_status", { hovered: true });
        };

        const handleMouseLeave = () => {
            invoke("update_hover_status", { hovered: false });

            if (!isPinnedRef.current) {
                getCurrentWindow().hide();
            }
        };

        document.body.addEventListener("mouseenter", handleMouseEnter);
        document.body.addEventListener("mouseleave", handleMouseLeave);

        return () => {
            document.body.removeEventListener("mouseenter", handleMouseEnter);
            document.body.removeEventListener("mouseleave", handleMouseLeave);
        };
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
            appWindow.listen("tauri://blur", () => {
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
