import {Component, useEffect} from "react";
import "./AdminComponent.scss"
import {Outlet, useNavigate} from "react-router";
import {CloseOutlined, FullscreenOutlined, MinusOutlined, SettingOutlined} from "@ant-design/icons";
import {appWindow} from "@tauri-apps/api/window";
import {Avatar} from "antd";


export function AdminComponent() {

    const navigate = useNavigate();

    useEffect(() => {
        navigate("/admin/chat");
    });

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
                    <Outlet/>
                </div>
        </>
    );

}