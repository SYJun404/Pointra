import { TransResultTypes } from "../types/transResult";
import Loading from "./Loading";

function Sentence({ transResult }: { transResult: TransResultTypes | null }) {
    if (transResult === null) {
        return <Loading />;
    }
    const { translate } = transResult.data;

    return (
        <div className="mx-3 flex flex-col p-3 pt-2.5 flex-1 min-h-0 border-borderMainW border rounded-xl overflow-scroll no-scrollbar">
            <main className="flex flex-col gap-2 ">
                <div className="flex justify-between items-center">
                    <p className="text-mainTitleW">{translate.text}</p>
                </div>
                <div className="border-t border-borderSubW my-1"></div>

                <div className="flex flex-col mt-0.5">
                    <p className=" text-mainBlueW">{translate.dit}</p>
                </div>
            </main>
        </div>
    );
}

export default Sentence;
