import {Component} from "react";
import "./admin.scss"
import {Outlet} from "react-router";
import {CloseOutlined, FullscreenOutlined, MinusOutlined, SettingOutlined} from "@ant-design/icons";
import {appWindow} from "@tauri-apps/api/window";


export class AdminComponent extends Component {
    state = {
        hasMounted: false
    };


    constructor(props: AdminComponent) {
        super(props);
    }


    closeClick() {
        appWindow.minimize().then();
    }

    quickClick() {
        appWindow.hide().then();
    }
    full(){
        appWindow.isMaximized().then(b => {
            if (b) {
                appWindow.unmaximize().then();
            } else {
                appWindow.maximize().then();
            }
        })
    }

    render() {
        return (
            <>
                <div data-tauri-drag-region className={"title-bar flex"}>
                    <div className={"btn-group setting"}>
                        <SettingOutlined/>
                    </div>
                    <div className={"btn-group close"} onClick={this.closeClick}>
                        <MinusOutlined/>
                    </div>
                    <div className={"btn-group full"} onClick={this.full}>
                        <FullscreenOutlined />
                    </div>
                    <div className={"btn-group quit"} onClick={this.quickClick}>
                        <CloseOutlined/>
                    </div>
                </div>
                <div className={"admin-content"}>
                    <Outlet/>
                </div>
            </>
        );
    }

}