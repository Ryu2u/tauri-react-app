import {useEffect} from "react";
import "./AdminComponent.scss"
import {Outlet, useNavigate} from "react-router";
import {CloseOutlined, FullscreenOutlined, MinusOutlined, SettingOutlined} from "@ant-design/icons";
import {appWindow} from "@tauri-apps/api/window";
import {Avatar} from "antd";
import {invoke} from "@tauri-apps/api";
import {User} from "../entity/Entity";
import {USER_KEY} from "../common/constant";


export function AdminComponent() {

    const navigate = useNavigate();

    useEffect(() => {
        console.log("connecting to websocket")
        invoke('get_user_info', {}).then((res: R) => {
            if (res.code == 200) {
                let user: User = res.data;
                localStorage.setItem(USER_KEY, user.id.toString());
                invoke('connect_websocket', {}).then();
            }
        });
    }, []);


    function closeClick() {
        appWindow.minimize().then();
    }

    function quickClick() {
        appWindow.hide().then();
    }

    function full() {
        appWindow.isMaximized().then(b => {
            if (b) {
                appWindow.unmaximize().then();
            } else {
                appWindow.maximize().then();
            }
        })
    }

    function routeToChat() {
        navigate('/admin/chat');
    }

    return (
        <>
            <div data-tauri-drag-region className={"title-bar flex"}>
                <div className={"btn-group setting"}>
                    <SettingOutlined/>
                </div>
                <div className={"btn-group close"} onClick={closeClick}>
                    <MinusOutlined/>
                </div>
                <div className={"btn-group full"} onClick={full}>
                    <FullscreenOutlined/>
                </div>
                <div className={"btn-group quit"} onClick={quickClick}>
                    <CloseOutlined/>
                </div>
            </div>
            <div className={"admin-content"}>
                <div className={"admin-tool-bar"}>
                    <ul>
                        <li onClick={routeToChat}>
                            <Avatar size={25}
                                    src={"https://ryu2u-1305537946.cos.ap-nanjing.myqcloud.com/pictures%2FQQ%E5%9B%BE%E7%89%8720231118112223.jpg"}
                            />
                        </li>
                    </ul>
                </div>
                <Outlet/>
            </div>
        </>
    );

}