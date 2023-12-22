import "./LoginComonent.scss";
import {CloseOutlined, LockOutlined, MinusOutlined, UserOutlined} from "@ant-design/icons";
import {Avatar, Button, message, Checkbox, Form, Input, FormInstance} from "antd";
import {appWindow} from "@tauri-apps/api/window";
import {invoke} from "@tauri-apps/api";
import {R} from "../entity/Entity";
import {useEffect, useRef, useState} from "react";
import {USER_AVATAR_PATH, USER_KEY} from "../common/constant";

export function LoginComponent() {

    const [messageApi, contextHolder] = message.useMessage();
    const formRef = useRef<FormInstance>(null);
    const [avatar, setAvatar] = useState('');

    useEffect(() => {
        let userKey = localStorage.getItem(USER_KEY);
        let user_avatar = localStorage.getItem(USER_AVATAR_PATH);
        if (user_avatar) {
            setAvatar(user_avatar);
        }
        if (userKey) {
            let key = parseInt(userKey);
            invoke('check_login', {
                userId: key
            }).then();
        }

        let username = localStorage.getItem("username");
        if (username) {
            formRef.current!.setFieldValue("username", username);
        }

    }, []);

    function closeClick() {
        appWindow.minimize().then();
    }

    function quickClick() {
        appWindow.hide().then();
    }

    function finished(value: any) {
        console.log("value ");
        console.log(value);
        const username = value['username'];
        const password = value['password'];
        const rememberMe = value['remember'];
        messageApi.open({
            key: 'login',
            type: 'loading',
            content: '正在登录...',
            style: {
                marginTop: '20px'
            },
            duration: 10
        });
        invoke('login', {
            username: username,
            password: password,
            rememberMe: rememberMe
        }).then((res: R) => {
                console.log(res);
                if (res.code == 200) {
                    messageApi.open({
                        key: 'login',
                        type: 'success',
                        content: '登录成功!',
                        style: {
                            marginTop: '20px'
                        }
                    });
                    localStorage.setItem("username", username);
                    setTimeout(() => {
                        invoke('route_to_admin', {}).then();
                    }, 500);
                } else {
                    messageApi.open({
                        key: 'login',
                        type: 'error',
                        content: res.msg,
                        style: {
                            marginTop: '20px'
                        }
                    });
                }
            }
        ).catch(err => {
            console.log(err);
            messageApi.open({
                key: 'login',
                type: 'error',
                content: '登录失败',
                style: {
                    marginTop: '20px'
                }
            });
        });
    }

    return (
        <>
            {contextHolder}
            <div data-tauri-drag-region className={"title-bar flex"}>
                <div className={"btn-group close"} onClick={closeClick}>
                    <MinusOutlined/>
                </div>
                <div className={"btn-group quit"} onClick={quickClick}>
                    <CloseOutlined/>
                </div>
            </div>
            <div className={"login-div"}>
                <div data-tauri-drag-region className={"avatar-div"}>
                    <Avatar className={"user-avatar"} size={70} src={avatar}/>
                </div>
                <div className={"input-div"}>
                    <Form
                        ref={formRef}
                        className="login-form"
                        initialValues={{remember: true}}
                        onFinish={finished}
                    >
                        <Form.Item
                            name="username"
                            rules={[{required: true, message: 'Please input your Username!'}]}
                        >
                            <Input prefix={<UserOutlined className="site-form-item-icon"/>} autoComplete="off" placeholder="Username"/>
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