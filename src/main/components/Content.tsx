import { Volume, Ellipsis } from "@gravity-ui/icons";
import { TransResultTypes, UsualDict } from "../types/transResult";
import { Skeleton } from "@heroui/react";

function Content({ transResult }: { transResult: TransResultTypes | null }) {
    if (transResult === null) {
        return (
            <div className="mx-3 flex flex-col p-3 pt-2.5 flex-1 min-h-0 border-borderMainW border rounded-xl">
                <div className="shadow-panel w-62.5 space-y-5 rounded-lg bg-transparent p-4">
                    <Skeleton className="h-32 rounded-lg" />
                    <div className="space-y-3">
                        <Skeleton className="h-3 w-3/5 rounded-lg" />
                        <Skeleton className="h-3 w-4/5 rounded-lg" />
                        <Skeleton className="h-3 w-2/5 rounded-lg" />
                    </div>
                </div>
            </div>
        );
    }

    const { wordCard, voice, translate } = transResult.data;
    console.log(transResult.data);

    const posStyles: Record<string, string> = {
        "n.": "bg-blue-50 border border-blue-200 text-blue-600",
        "v.": "bg-green-50 border border-green-200 text-green-600",
        "adj.": "bg-purple-50 border border-purple-200 text-purple-600",
        "adv.": "bg-amber-50 border border-amber-200 text-amber-600",
    };

    const getPosStyle = (pos: string) =>
        posStyles[pos] ?? "bg-gray-100 border border-gray-200 text-gray-500";

    const UsualDict = ({ dict }: { dict: UsualDict[] }) => (
        <div className="border-t border-borderSubW pt-1 mt-2 flex flex-1 min-h-0 flex-col gap-2 overflow-y-auto no-scrollbar">
            {dict.map((entry) => (
                <div key={entry.pos} className="shrink-0">
                    {" "}
                    <span
                        className={`inline-block text-[10px] font-semibold px-1.5 pb-0.5  rounded mb-1.5 ${getPosStyle(entry.pos)}`}
                    >
                        {entry.pos}
                    </span>
                    <div className="flex flex-wrap gap-1">
                        {entry.values[0]
                            .split("；")
                            .slice(0, 10)
                            .map((val) => (
                                <span
                                    key={val}
                                    className="bg-gray-50 border pt-1 border-gray-200 rounded text-[11px] text-gray-600 px-1.5 py-0.5"
                                >
                                    {val}
                                </span>
                            ))}
                    </div>
                </div>
            ))}
        </div>
    );

    return (
        <div className="mx-3 flex flex-col p-3 pt-2.5 flex-1 min-h-0 border-borderMainW border rounded-xl">
            <main className="flex flex-col gap-2 ">
                <div className="flex justify-between items-center">
                    <p className="text-xl text-mainTitleW">{translate.text}</p>
                    <p
                        className="flex items-center justify-center w-6 h-6 rounded-md bg-blueBgW border border-blueBorderW
                                   cursor-pointer transition-transform active:scale-90"
                    >
                        <Ellipsis color="#4a90d9" width={14} height={14} />
                    </p>
                </div>

                {typeof voice === "string" ? null : (
                    <div className="flex justify-between items-center">
                        <p className="flex font-sans font-semibold  items-center text-xs text-tagSecondW justify-center h-6 px-2 rounded-md bg-tagBgW border border-borderMainW">
                            {
                                voice.phonetic.find(
                                    (item) => item.type === "usa",
                                )?.text
                            }
                        </p>
                        <p
                            className="flex items-center justify-center w-6 h-6 rounded-md bg-tagBgW border border-borderMainW
                                       cursor-pointer transition-transform active:scale-90"
                        >
                            <Volume color="#aaaaaa" width={14} height={14} />
                        </p>
                    </div>
                )}

                <div className="border-t border-borderSubW my-1"></div>

                {/*常用释义*/}
                <div className="flex flex-col mt-0.5">
                    <p className="text-[10px] text-tagW font-sans">常用释义</p>
                    <p className="text-xl text-mainBlueW">{translate.dit}</p>
                </div>
            </main>

            {wordCard.show && <UsualDict dict={wordCard.usualDict} />}
        </div>
    );
}

export default Content;
