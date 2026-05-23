import "../assets/css/App.css";
import { useWindowListener } from "./utils/useCustom";
import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";
import { Routes, useNavigate, Route } from "react-router-dom";
import useUiStore from "./store/useUiStore";
import SearchPage from "./pages/SearchPage";
import HomePage from "./pages/HomePage";

function App() {
    const navigate = useNavigate();
    const isPinned = useUiStore((state) => state.isPinned);
    const setIsPinned = useUiStore((state) => state.setIsPinned);

    // 监听鼠标移入/出窗口
    useWindowListener(isPinned, setIsPinned);

    useEffect(() => {
        const setupListener = async () => {
            const unlisten = await listen<string>(
                "win-router",
                async (event) => {
                    if (event.payload === "search") {
                        navigate("/search");
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
        <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/search" element={<SearchPage />} />
        </Routes>
    );
}

export default App;
