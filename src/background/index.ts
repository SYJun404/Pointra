import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { invoke } from "@tauri-apps/api/core";

class BackgroundManager {
    private static instance: BackgroundManager;
    private mainWindow: WebviewWindow | null = null;

    private constructor() {}

    // 单例模式，确保全局只有一个后台管理器
    public static getInstance(): BackgroundManager {
        if (!BackgroundManager.instance) {
            BackgroundManager.instance = new BackgroundManager();
        }

        return BackgroundManager.instance;
    }

    /**
     * 启动初始化
     */
    public async run() {
        await this.createMainWindow();
    }

    /**
     * 创建/获取主窗口
     */
    private async createMainWindow() {
        this.mainWindow = await WebviewWindow.getByLabel("main");

        if (!this.mainWindow) {
            this.mainWindow = new WebviewWindow("main", {
                title: "Pointra",
                url: "src/main/index.html",
                width: 300,
                height: 420,
                visible: false,
                resizable: false,
                decorations: false,
                center: true,
                transparent: true,
                shadow: true,
            });

            this.mainWindow.once("tauri://webview-created", async () => {
                try {
                    // 1. 告诉 Rust 给这个窗口施加 macOS 效果
                    // 注意：Tauri 2.0 会自动将当前窗口实例传递给 Rust 的 window 参数
                    await invoke("apply_window_effects", {
                        // 明确指定是对哪个窗口操作
                        label: "main",
                    });

                    // 2. 显示窗口
                    await this.mainWindow?.show();
                } catch (e) {
                    console.error("Failed to apply effects:", e);
                }
            });
        }
    }
}

// 执行
BackgroundManager.getInstance().run().catch(console.error);
