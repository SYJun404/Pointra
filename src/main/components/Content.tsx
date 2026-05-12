import { Volume } from "@gravity-ui/icons";
import { TransResultTypes } from "../types/transResult";
import { Skeleton } from "@heroui/react";

function Content({ transResult }: { transResult: TransResultTypes | null }) {
    if (transResult === null) {
        return (
            <div className="shadow-panel w-62.5 space-y-5 rounded-lg bg-transparent p-4">
                <Skeleton className="h-32 rounded-lg" />
                <div className="space-y-3">
                    <Skeleton className="h-3 w-3/5 rounded-lg" />
                    <Skeleton className="h-3 w-4/5 rounded-lg" />
                    <Skeleton className="h-3 w-2/5 rounded-lg" />
                </div>
            </div>
        );
    }

    const { wordCard, voice, translate } = transResult.data;

    return (
        <div className="mx-3 p-3 flex-1 border-borderMainW border rounded-xl">
            <main className="flex flex-col gap-2">
                <div className="flex justify-between items-center">
                    <p className="text-xl text-mainTitleW">
                        {translate.orig_text}
                    </p>
                    <p className="flex items-center text-xs text-mainBlueW justify-center h-5 pl-2.5 pr-2  rounded-md bg-blueBgW border border-blueBorderW">
                        n.
                    </p>
                </div>

                <div className="flex justify-between items-center">
                    <p className="flex font-sans font-semibold  items-center text-xs text-tagSecondW justify-center h-6 px-2 rounded-md bg-tagBgW border border-borderMainW">
                        /ˌser.ənˈdɪp.ɪ.ti/
                    </p>

                    <p
                        className="flex items-center justify-center w-6 h-6 rounded-md bg-tagBgW border border-borderMainW
                                   cursor-pointer transition-transform active:scale-90"
                    >
                        <Volume color="#aaaaaa" width={14} height={14} />
                    </p>
                </div>
                <div className="border-t border-borderSubW my-1"></div>

                <div>{translate.dit}</div>
            </main>
        </div>
    );
}

export default Content;
