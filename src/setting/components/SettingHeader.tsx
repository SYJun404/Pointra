import { CircleInfo, Gear, Xmark } from "@gravity-ui/icons";
import type { UseShortcutManagerReturn } from "../hooks";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function SettingToast({
    toastMsg,
}: {
    toastMsg: UseShortcutManagerReturn["toastMsg"];
}) {
    if (!toastMsg) return null;
    return (
        <div className="fixed top-4 left-1/2 -translate-x-1/2 z-50 animate-in fade-in slide-in-from-top-2 duration-200">
            <div className="flex items-center gap-2 px-4 py-2 rounded-xl bg-gray-900 text-white text-sm shadow-lg border border-gray-700">
                <CircleInfo width={14} height={14} color="#fff" />
                <span>{toastMsg}</span>
            </div>
        </div>
    );
}

export function SettingHeader() {
    const closeApp = () => {
        getCurrentWindow().close();
    };

    return (
        <>
            <div data-tauri-drag-region className="h-3 w-full absolute" />
            <div className="p-3  flex items-center justify-between border-b border-borderSubW shrink-0">
                <div className="flex items-center gap-2">
                    <div className="flex items-center justify-center w-7 h-7 rounded-lg bg-blueBgW border border-blueBorderW">
                        <Gear width={14} height={14} color="#4a90d9" />
                    </div>
                    <h1 className="text-lg font-medium text-mainTitleW">
                        Settings
                    </h1>
                </div>

                <div
                    onClick={closeApp}
                    className="flex items-center justify-center w-7 h-7 rounded-lg border bg-red-50 border-red-200    cursor-pointer transition-transform active:scale-90"
                >
                    <Xmark width={14} height={14} color="#fa2c37" />
                </div>
            </div>
        </>
    );
}
