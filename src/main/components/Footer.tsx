import { Clock, Gear, Magnifier } from "@gravity-ui/icons";

function Footer() {
    const ACTION_BUTTONS = [
        {
            id: "search",
            icon: <Magnifier color="#bbbbbb" height={14} width={14} />,
        },
        {
            id: "settings",
            icon: <Gear color="#bbbbbb" height={14} width={14} />,
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
                <Clock color="#bbbbbb" width={16} height={16} />
                <p className="text-[11px] text-tagW absolute left-5 top-1">
                    今日已翻译
                    <text className="text-mainBlueW"> 24 </text>次
                </p>

                <div className="ml-auto flex gap-1.5">
                    {ACTION_BUTTONS.map(({ id, icon }) => (
                        <IconButton
                            key={id}
                            onClick={() => console.log(`${id} clicked`)}
                        >
                            {icon}
                        </IconButton>
                    ))}
                </div>
            </div>
        </div>
    );
}

export default Footer;
