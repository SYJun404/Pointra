import logo from "../../assets/icon/pointraInApp.png";

function Header() {
    return (
        <div className="flex mx-3 items-center relative">
            <img className="w-6 h-6" src={logo}></img>
            <p className="absolute left-8 top-px  text-mainTitleW">Pointra</p>

            <div className="ml-auto flex items-center gap-1">
                <p className="h-2 w-2 rounded-full bg-stateGreenW"></p>
                <p className="text-[11px] pt-px text-stateGreenW ">监听中</p>
            </div>
        </div>
    );
}

export default Header;
