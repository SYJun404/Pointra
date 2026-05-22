import { Pin, Magnifier, ClockArrowRotateLeft } from "@gravity-ui/icons";
import logo from "../../assets/icon/pointraInApp.png";
import useUiStore from "../../main/store/useUiStore";
import { useNavigate } from "react-router-dom";

function Footer({ path }: { path: string }) {
    const navigate = useNavigate();
    const isPinned = useUiStore((state) => state.isPinned);
    const setIsPinned = useUiStore((state) => state.setIsPinned);

    const ACTION_BUTTONS = [
        {
            id: "search",
            router: "/",
            icon: (
                <ClockArrowRotateLeft color="#bbbbbb" height={15} width={15} />
            ),
        },
        {
            id: "home",
            router: "/search",
            icon: <Magnifier color="#bbbbbb" height={14} width={14} />,
        },
    ];

    const IconButton = ({
        children,
        onClick,
    }: {
        children: React.ReactNode;
        onClick: () => void;
    }) => (
        <div
            onClick={onClick}
            className="flex items-center justify-center w-6 h-6 rounded-md bg-white border border-borderMainW
                       cursor-pointer transition-transform active:scale-90"
        >
            {children}
        </div>
    );

    return (
        <div className="mt-auto bg-subBgW h-10 px-3 py-2 rounded-b-3xl border-t border-borderSubW">
            <div className="flex items-center h-full relative gap-2">
                <img className="w-4 h-4" src={logo}></img>
                <p className="text-sm text-tagW absolute left-6">Pointra</p>

                <div className="ml-auto flex gap-1.5">
                    {(() => {
                        const activeBtn = ACTION_BUTTONS.find(
                            ({ id }) => id === path,
                        );
                        if (!activeBtn) return null;

                        const { id, icon, router } = activeBtn;
                        return (
                            <IconButton
                                key={id}
                                onClick={() => navigate(router)}
                            >
                                {icon}
                            </IconButton>
                        );
                    })()}
                    <div
                        onClick={() => setIsPinned(!isPinned)}
                        className={`
                            flex items-center justify-center w-6 h-6 rounded-md border cursor-pointer
                            transition-all duration-200 active:scale-90
                            ${
                                isPinned
                                    ? "bg-red-50 border-red-200" // 选中状态
                                    : "bg-white  border-borderMainW" // 默认状态
                            }
                          `}
                    >
                        <div
                            className={`transition-transform duration-300 ${!isPinned ? "rotate-0" : "-rotate-45"}`}
                        >
                            <Pin
                                // 根据状态切换颜色
                                color={isPinned ? "#fa2c37" : "#bbbbbb"}
                                width={14}
                                height={14}
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default Footer;
