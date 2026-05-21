import { Magnifier, Xmark } from "@gravity-ui/icons";
import { useState, useRef } from "react";
import Header from "../components/Header";
import Footer from "../components/Footer";
import { TransResultZHTypes } from "../types/transResult";
import { invoke } from "@tauri-apps/api/core";
import SearchContent from "../components/SearchContent";
import { useWindowListener, useOnWindowChange } from "../utils/useCustom";
import useUiStore from "../store/useUiStore";
import { useNavigate } from "react-router-dom";

function SearchPage() {
    const navigate = useNavigate();
    const inputRef = useRef<HTMLInputElement>(null);
    const [input, setinput] = useState<string>("");
    const [results, setResults] = useState<TransResultZHTypes | null>(null);
    const isPinned = useUiStore((state) => state.isPinned);

    // 监听鼠标移入/出窗口
    useWindowListener(isPinned);
    // 窗口显示时执行的回调
    useOnWindowChange(() => {
        navigate("/");
    });

    const handleSubmit = async (e: any) => {
        e.preventDefault();
        inputRef.current?.blur();
        const res = await invoke<TransResultZHTypes>("fetch_trans_res", {
            word: input,
        });
        if (res.status === 200) {
            console.log(res);
            setResults(res);
        } else {
            setResults(null);
        }
    };

    return (
        <div className="pt-3 flex flex-col gap-3 h-screen overflow-hidden">
            <Header />
            {/* 顶部栏 */}
            <div className="mx-3 flex items-center gap-3">
                <div className="flex-1 relative">
                    <form onSubmit={handleSubmit}>
                        <input
                            type="text"
                            ref={inputRef}
                            value={input}
                            onChange={(e: any) => setinput(e.target.value)}
                            placeholder="Translate Anything..."
                            className="w-full h-9 pl-9 pr-9 text-sm rounded-lg bg-white border border-borderMainW
                                   outline-none focus:border-mainBlueW transition-colors text-mainTitleW"
                        />
                    </form>
                    <Magnifier
                        color="#bbbbbb"
                        height={14}
                        width={14}
                        className="absolute left-3 top-1/2 -translate-y-1/2"
                    />
                    <Xmark
                        onClick={() => {
                            setinput("");
                            setResults(null);
                        }}
                        color="#bbbbbb"
                        height={14}
                        width={14}
                        className="absolute right-3 top-1/2 -translate-y-1/2 cursor-pointer"
                    />
                </div>
            </div>

            {/* 历史搜索/搜索结果区域 */}
            <div className="mx-3 flex-1  border border-borderMainW rounded-xl overflow-y-auto no-scrollbar">
                {results ? (
                    typeof results.data.wordCard.secondQuery === "string" ? (
                        <p className="text-sm text-tagW text-center mt-20">
                            {results.data.translate.dit}
                        </p>
                    ) : (
                        <SearchContent
                            results={results.data.wordCard.secondQuery}
                        />
                    )
                ) : (
                    <p className="text-sm text-tagW text-center mt-20">
                        输入关键词开始搜索
                    </p>
                )}
            </div>
            <Footer path={"search"} />
        </div>
    );
}

export default SearchPage;
