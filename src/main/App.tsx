import "../assets/css/App.css";
import { useWindowListener, useOnWindowChange } from "./utils/useCustom";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import Header from "./components/Header";
import Content from "./components/Content";
import Footer from "./components/Footer";
import { TransResultTypes } from "./types/transResult";
import ApiError from "./components/ApiError";
import { judgeSentence } from "./utils/tool";
import Sentence from "./components/Sentence";
import useUiStore from "./store/useUiStore";

function App() {
    const [transResult, setTransResult] = useState<TransResultTypes | null>(
        null,
    );
    const [error, setError] = useState<string | null>(null);
    const [isSentence, setisSentence] = useState(false);
    const isPinned = useUiStore((state) => state.isPinned);

    // 监听鼠标移入/出窗口
    useWindowListener(isPinned);
    // 窗口显示时执行的回调
    useOnWindowChange(() => {
        setTransResult(null);
        setError(null);
    });

    useEffect(() => {
        const setupListener = async () => {
            const unlisten = await listen<string>(
                "from-cursor",
                async (event) => {
                    const res = await invoke<TransResultTypes>(
                        "fetch_trans_res",
                        {
                            word: event.payload,
                        },
                    );
                    if (res.status === 200) {
                        console.log(res);
                        setTransResult(res);
                        setisSentence(judgeSentence(res.data.translate.text));
                    } else {
                        setError(res.msg);
                    }
                },
            );

            // 返回清理函数
            return unlisten;
        };

        const listenerPromise = setupListener();

        return () => {
            listenerPromise.then((unlisten) => unlisten());
        };
    }, []);

    return (
        <div className="pt-3 flex flex-col gap-3 h-screen overflow-hidden">
            <Header />

            {error === null ? (
                isSentence ? (
                    <Sentence transResult={transResult} />
                ) : (
                    <Content transResult={transResult} />
                )
            ) : (
                <ApiError message={error} />
            )}

            <Footer />
        </div>
    );
}

export default App;
