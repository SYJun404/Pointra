import { ArrowRightArrowLeft } from "@gravity-ui/icons";

export function AboutSection() {
    return (
        <section>
            <div className="flex items-center justify-between mb-3">
                <div className="flex items-center gap-2">
                    <div className="w-1 h-4 rounded-full bg-mainBlueW" />
                    <h2 className="text-sm font-medium text-mainTitleW">
                        关于
                    </h2>
                </div>
                <p className="text-[11px] text-tagSecondW mt-px rounded-md ">
                    About
                </p>
            </div>
            <div className="rounded-xl border border-borderMainW bg-white overflow-hidden px-4 py-3.5">
                <div className="flex items-center gap-3">
                    <div className="flex items-center justify-center w-9 h-9 rounded-lg bg-blueBgW border border-blueBorderW">
                        <ArrowRightArrowLeft
                            width={16}
                            height={16}
                            color="#4a90d9"
                        />
                    </div>
                    <div>
                        <p className="text-sm text-mainTitleW font-medium">
                            Pointra
                        </p>
                        <p className="text-[11px] text-tagSecondW mt-0.5">
                            版本 0.1.0 —— 光标所指 · 翻译即达
                        </p>
                    </div>
                </div>
            </div>
            <p className="text-xs mt-3 text-center text-tagSecondW">
                Developed By SYJun
            </p>
        </section>
    );
}
