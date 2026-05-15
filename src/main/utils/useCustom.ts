import { useEffect, useRef } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

export function useWindowListener() {
    useEffect(() => {
        // 监听鼠标进入窗口
        document.body.addEventListener("mouseenter", () => {
            invoke("update_hover_status", { hovered: true });
        });
        // 监听鼠标离开窗口
        document.body.addEventListener("mouseleave", () => {
            invoke("update_hover_status", { hovered: false });
        });
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
