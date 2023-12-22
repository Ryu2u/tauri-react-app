import {useEffect, useState} from "react";
import "./AdminComponent.scss"
import {Outlet, useNavigate} from "react-router";
import {CloseOutlined, FullscreenOutlined, LogoutOutlined, MinusOutlined, SettingOutlined} from "@ant-design/icons";
import {appWindow} from "@tauri-apps/api/window";
import {Avatar, Dropdown, MenuProps} from "antd";
import {invoke} from "@tauri-apps/api";
import {User} from "../entity/Entity";
import {USER_AVATAR_PATH, USER_KEY} from "../common/constant";


export function AdminComponent() {

    const navigate = useNavigate();
    const [avatar, setAvatar] = useState('');

    useEffect(() => {
        console.log("connecting to websocket")
        invoke('get_user_info', {}).then((res: R) => {
            if (res.code == 200) {
                let user: User = res.data;
                localStorage.setItem(USER_KEY, user.id.toString());
                localStorage.setItem(USER_AVATAR_PATH, user.avatarPath);
                setAvatar(user.avatarPath);
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

    const onClick: MenuProps['onClick'] = ({key}) => {
        switch (key) {
            case 'logout': {
                // logout
                console.log(key);

                invoke('logout').then()

                break;
            }
            case '0': {
                break;
            }
            default: {
                console.log(key);
            }
        }
    };


    const items: MenuProps['items'] = [
        {
            label: 'resolve',
            key: '0',
        },
        {
            type: 'divider',
        },
        {
            label: <span>
                <LogoutOutlined/> 退出登录
            </span>,
            key: 'logout',
        },
    ];

    return (
        <>
            <div data-tauri-drag-region className={"title-bar flex"}>
                <Dropdown menu={{items, onClick}} trigger={['click']}>
                    <div className={"btn-group setting"}>
                        <SettingOutlined/>
                    </div>
                </Dropdown>
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
                                    src={avatar}
                            />
                        </li>
                    </ul>
                </div>
                <Outlet/>
            </div>
        </>
    );

}