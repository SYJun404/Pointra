import "../assets/css/App.css";
import { useShortcutManager } from "./hooks";
import { SettingToast, SettingHeader } from "./components/SettingHeader";
import { GeneralSection } from "./components/GeneralSection";
import { ShortcutSection } from "./components/ShortcutSection";
import { AboutSection } from "./components/AboutSection";

function SettingPage() {
    const {
        shortcuts,
        generalSettings,
        hasChanges,
        recordingId,
        recordingRef,
        conflictIds,
        toastMsg,
        toggleGeneral,
        startRecording,
        cancelRecording,
        handleKeyDown,
        resetToDefault,
        resetAllShortcuts,
        handleSave,
    } = useShortcutManager();

    return (
        <div className="h-screen rounded-4xl pb-3 flex flex-col bg-white overflow-hidden select-none">
            <SettingToast toastMsg={toastMsg} />
            <SettingHeader />

            {/* ============ 滚动内容 ============ */}
            <div className="flex-1 overflow-y-auto no-scrollbar px-3 pt-3  space-y-6">
                <GeneralSection
                    settings={generalSettings}
                    onToggle={toggleGeneral}
                />

                <ShortcutSection
                    shortcuts={shortcuts}
                    recordingId={recordingId}
                    recordingRef={recordingRef}
                    conflictIds={conflictIds}
                    onStartRecording={startRecording}
                    onCancelRecording={cancelRecording}
                    onKeyDown={handleKeyDown}
                    onResetOne={resetToDefault}
                    onResetAll={resetAllShortcuts}
                />

                <AboutSection />
            </div>

            {/* ============ 底部安全区 ============ */}
            {hasChanges && (
                <div className="shrink-0 p-3 pb-0 border-t border-borderSubW bg-subBgW">
                    <button
                        onClick={handleSave}
                        className="w-full h-10 rounded-full bg-mainBlueW text-white text-sm font-medium cursor-pointer
                                   hover:brightness-110 active:scale-90 transition-all duration-150 shadow-sm"
                    >
                        保存设置
                    </button>
                </div>
            )}
        </div>
    );
}

export default SettingPage;
