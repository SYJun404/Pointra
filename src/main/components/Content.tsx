import { Volume } from "@gravity-ui/icons";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

function Content() {
    useEffect(() => {
        invoke("query_word", { word: "test" })
            .then((result) => {
                console.log(result);
            })
            .catch((error) => console.error(error));
    }, []);

    return (
        <div className="mx-3 p-3 flex-1 border-borderMainW border rounded-xl">
            <main className="flex flex-col gap-2">
                <div className="flex justify-between items-center">
                    <p className="text-xl text-mainTitleW">serendipity</p>
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
            </main>
        </div>
    );
}

export default Content;
