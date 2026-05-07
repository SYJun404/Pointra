import logo from "../../assets/icon/pointraInApp.png";

function Header() {
    return (
        <div className="flex items-center relative">
            <img className="w-6 h-6" src={logo}></img>
            <p className="absolute left-8 top-px font-DingTalk text-mainTitleW">
                Pointra
            </p>

            <p className="ml-auto">text</p>
        </div>
    );
}

export default Header;
