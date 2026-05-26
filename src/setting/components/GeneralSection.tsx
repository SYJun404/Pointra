import type { GeneralSetting } from "../types";

export function GeneralSection({
    settings,
    onToggle,
}: {
    settings: GeneralSetting[];
    onToggle: (id: string) => void;
}) {
    return (
        <section>
            <div className="flex items-center justify-between mb-3">
                <div className="flex items-center gap-2">
                    <div className="w-1 h-4 rounded-full bg-mainBlueW" />
                    <h2 className="text-sm font-medium text-mainTitleW">
                        通用设置
                    </h2>
                </div>
                <p className="text-[11px] text-tagSecondW mt-px rounded-md ">
                    General
                </p>
            </div>

            <div className="rounded-xl border border-borderMainW divide-y divide-borderSubW bg-white overflow-hidden">
                {settings.map((item) => (
                    <div
                        key={item.id}
                        className="flex items-center justify-between px-4 py-3.5 hover:bg-subBgW transition-colors"
                    >
                        <div className="flex-1 min-w-0 mr-3">
                            <p className="text-sm text-mainTitleW">
                                {item.label}
                            </p>
                            <p className="text-[11px] text-tagSecondW mt-0.5">
                                {item.description}
                            </p>
                        </div>
                        <label className="relative inline-flex items-center cursor-pointer shrink-0">
                            <input
                                type="checkbox"
                                className="sr-only peer"
                                checked={item.enabled}
                                onChange={() => onToggle(item.id)}
                            />
                            <div
                                className={`w-9 h-5 rounded-full transition-colors duration-200 ${
                                    item.enabled
                                        ? "bg-mainBlueW"
                                        : "bg-gray-200"
                                }`}
                            >
                                <div
                                    className={`w-3.5 h-3.5 rounded-full bg-white shadow-sm transition-transform duration-200 ${
                                        item.enabled
                                            ? "translate-x-[19px]"
                                            : "translate-x-1"
                                    }`}
                                    style={{ marginTop: "3px" }}
                                />
                            </div>
                        </label>
                    </div>
                ))}
            </div>

            <p className="text-[10px] text-tagSecondW mt-2 text-center">
                部分设置需要重启后生效
            </p>
        </section>
    );
}
