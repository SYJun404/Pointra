import "../assets/css/App.css";
import { useWindowShortcut, useOnWindowChange } from "./utils/useCustom";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import Header from "./components/Header";
import Content from "./components/Content";
import Footer from "./components/Footer";
import { TransResultTypes } from "./types/transResult";

function App() {
    const [transResult, setTransResult] = useState<TransResultTypes | null>(
        null,
    );

    // 按下Esc隐藏当前窗口
    useWindowShortcut();
    // 窗口显示时执行的回调
    useOnWindowChange(() => {
        setTransResult(null);
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
                        setTransResult(res);
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
            <Content transResult={transResult} />
            <Footer />
        </div>
    );
}

export default App;
