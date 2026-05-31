import type { GeneralSetting } from "../types";
import { Switch, Slider, ListBox, Select } from "@heroui/react";
import type { Key } from "@heroui/react";

export function GeneralSection({
    settings,
    onToggle,
    onUpdate,
}: {
    settings: GeneralSetting[];
    onToggle: (id: string) => void;
    onUpdate: (id: string, value: boolean | string | number) => void;
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
                {settings.map((item) => {
                    // --- 单词发音：下拉选择器 ---
                    if (item.id === "pronunciation") {
                        return (
                            <div
                                key={item.id}
                                className="flex items-center justify-between p-3 hover:bg-subBgW transition-colors"
                            >
                                <div className="flex-1 min-w-0 mr-3">
                                    <p className="text-sm text-mainTitleW">
                                        {item.label}
                                    </p>
                                    <p className="text-[11px] text-tagSecondW mt-0.5">
                                        {item.description}
                                    </p>
                                </div>
                                <div className="relative inline-flex items-center cursor-pointer shrink-0">
                                    <Select
                                        aria-label="Select"
                                        className="w-20 "
                                        placeholder="Select"
                                        variant="secondary"
                                        value={item.value as Key}
                                        onChange={(value) =>
                                            onUpdate(item.id, value as string)
                                        }
                                    >
                                        <Select.Trigger className="rounded-lg ">
                                            <Select.Value className="text-sm" />
                                            <Select.Indicator />
                                        </Select.Trigger>
                                        <Select.Popover className="rounded-lg">
                                            <ListBox>
                                                <ListBox.Item
                                                    id="us"
                                                    textValue="美式"
                                                    className="rounded-[5px]"
                                                >
                                                    美式
                                                    <ListBox.ItemIndicator />
                                                </ListBox.Item>
                                                <ListBox.Item
                                                    id="uk"
                                                    textValue="英式"
                                                    className="rounded-[5px]"
                                                >
                                                    英式
                                                    <ListBox.ItemIndicator />
                                                </ListBox.Item>
                                            </ListBox>
                                        </Select.Popover>
                                    </Select>
                                </div>
                            </div>
                        );
                    }

                    // --- 发音音量：滑动条 ---
                    if (item.id === "pronunciation_volume") {
                        const volume = item.value as number;
                        return (
                            <div
                                key={item.id}
                                className="flex items-center justify-between p-3 hover:bg-subBgW transition-colors"
                            >
                                <div className="flex-1 min-w-0 mr-3">
                                    <p className="text-sm text-mainTitleW">
                                        {item.label}
                                    </p>
                                    <p className="text-[11px] text-tagSecondW mt-0.5">
                                        {item.description}
                                    </p>
                                </div>
                                <div className="relative inline-flex items-center cursor-pointer shrink-0">
                                    <Slider
                                        aria-label="Slider"
                                        className="w-30"
                                        value={volume}
                                        onChange={(value) => {
                                            const num = Number(value);
                                            if (num <= 10) {
                                                onUpdate(item.id, 10);
                                                return;
                                            }
                                            onUpdate(item.id, num);
                                        }}
                                    >
                                        <Slider.Output />
                                        <Slider.Track>
                                            <Slider.Fill />
                                            <Slider.Thumb />
                                        </Slider.Track>
                                    </Slider>
                                </div>
                            </div>
                        );
                    }

                    // --- 默认：开关切换 ---
                    return (
                        <div
                            key={item.id}
                            className="flex items-center justify-between p-3 hover:bg-subBgW transition-colors"
                        >
                            <div className="flex-1 min-w-0 mr-3">
                                <p className="text-sm text-mainTitleW">
                                    {item.label}
                                </p>
                                <p className="text-[11px] text-tagSecondW mt-0.5">
                                    {item.description}
                                </p>
                            </div>
                            <div className="relative inline-flex items-center cursor-pointer shrink-0">
                                <Switch
                                    isSelected={item.value as boolean}
                                    onChange={() => onToggle(item.id)}
                                >
                                    <Switch.Control>
                                        <Switch.Thumb />
                                    </Switch.Control>
                                </Switch>
                            </div>
                        </div>
                    );
                })}
            </div>
        </section>
    );
}
