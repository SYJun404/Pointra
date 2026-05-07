import "../assets/css/App.css";
import { useWindowShortcut, useOnWindowChange } from "./utils/useCustom";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import Header from "./components/header";

function App() {
    const [word, setword] = useState<string>("");

    // 按下Esc隐藏当前窗口
    useWindowShortcut();
    // 窗口显示时执行的回调
    useOnWindowChange(
        async () => {
            const wordOcr = await invoke<string>("get_word_under_cursor");
            setword(wordOcr);
        },
        () => {
            setword("");
        },
    );

    return (
        <div className="m-3">
            <Header />
        </div>
    );
}

export default App;
