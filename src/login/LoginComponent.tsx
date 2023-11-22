import {Component} from "react";
import "./LoginComonent.scss";
import {CloseOutlined, LockOutlined, MinusOutlined, UserOutlined} from "@ant-design/icons";
import {Avatar, Button, Checkbox, Form, Input} from "antd";
import {appWindow} from "@tauri-apps/api/window";
import {invoke} from "@tauri-apps/api";

export class LoginComponent extends Component {

    closeClick() {
        appWindow.minimize().then();
    }

    quickClick() {
        appWindow.hide().then();
    }

    finished(value: any) {
        console.log("value ");
        console.log(value);
        invoke('route_to_admin', {}).then();
    }

    render() {
        return (
            <>
                <div data-tauri-drag-region className={"title-bar flex"}>
                    <div className={"btn-group close"} onClick={this.closeClick}>
                        <MinusOutlined/>
                    </div>
                    <div className={"btn-group quit"} onClick={this.quickClick}>
                        <CloseOutlined/>
                    </div>
                </div>
                <div className={"login-div"}>
                    <div data-tauri-drag-region className={"avatar-div"}>
                        <Avatar className={"user-avatar"} size={70} src={"https://ryu2u-1305537946.cos.ap-nanjing.myqcloud.com/pictures%2FQQ%E5%9B%BE%E7%89%8720231118112223.jpg"}/>

                    </div>
                    <div className={"input-div"}>
                        <Form className="login-form"
                              initialValues={{remember: true}}
                              onFinish={this.finished}
                        >
                            <Form.Item
                                name="username"
                                rules={[{required: true, message: 'Please input your Username!'}]}
                            >
                                <Input prefix={<UserOutlined className="site-form-item-icon"/>} placeholder="Username"/>
                            </Form.Item>
                            <Form.Item
                                name="password"
                                rules={[{required: true, message: 'Please input your Password!'}]}
                            >
                                <Input.Password
                                    prefix={<LockOutlined className="site-form-item-icon"/>}
                                    type="password"
                                    placeholder="Password"
                                />
                            </Form.Item>
                            <Form.Item>
                                <Form.Item name="remember" valuePropName="checked" noStyle>
                                    <Checkbox>Remember me</Checkbox>
                                </Form.Item>

                                <a className="login-form-forgot" href="">
                                    Forgot password
                                </a>
                            </Form.Item>

                            <Form.Item>
                                <Button type="primary" htmlType="submit" className="login-btn">
                                    Log in
                                </Button>
                            </Form.Item>
                        </Form>
                    </div>
                </div>
            </>
        );
    }
}