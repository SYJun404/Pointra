import "../assets/css/App.css";
import { useWindowShortcut, useOnWindowShow } from "./utils/useCustom";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

function App() {
    const [word, setword] = useState<string>("");

    // 按下Esc隐藏当前窗口
    useWindowShortcut();
    // 窗口显示时执行的回调
    useOnWindowShow(async () => {
        const wordOcr = await invoke<string>("get_word_under_cursor");
        setword(wordOcr);
    });

    return (
        <div className="flex">
            <p>{word}</p>
        </div>
    );
}

export default App;
