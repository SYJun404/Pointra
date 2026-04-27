import { invoke } from "@tauri-apps/api/core";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

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

    // 启动初始化
    public async run() {
        await this.createMainWindow();
    }

    // 创建主窗口
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
                    await invoke("apply_window_effects", {
                        label: "main",
                    });

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
