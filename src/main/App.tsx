import "../assets/css/App.css";
import { useWindowShortcut } from "./utils/useWindowShortcut";

function App() {
    // 按下Esc隐藏当前窗口
    useWindowShortcut();

    return (
        <div className="flex">
            <p>index</p>
        </div>
    );
}

export default App;
