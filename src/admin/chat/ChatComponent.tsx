import {Component} from "react";
import {Avatar, Input, Layout, List} from "antd";
import Sider from "antd/es/layout/Sider";
import {Content, Footer, Header} from "antd/es/layout/layout";
import "./chat.scss"
import {SearchOutlined} from "@ant-design/icons";

export class ChatComponent extends Component {

    constructor(props: ChatComponent) {
        super(props);
        this.state = {
            searchBarVisible: false
        }
    }

    list: number[] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    roomList = this.list.map(v => {
        return (
            <>
                <List.Item key={v} className={"room-list-item"}>
                    <List.Item.Meta
                        avatar={<Avatar src={"https://ryu2u-1305537946.cos.ap-nanjing.myqcloud.com/pictures%2FQQ%E5%9B%BE%E7%89%8720231118112223.jpg"}/>}
                        title={"Hello World"}
                        description={"test test"}
                    >
                    </List.Item.Meta>
                </List.Item>
            </>
        );
    });

    sideMouseUpEvent(event: React.WheelEvent<HTMLDivElement>) {
        const sidebar = document.getElementById("side-bar")!;
        if (sidebar) {
            console.log(sidebar.scrollTop);
            if (sidebar.scrollTop == 0) {
                this.setState((prevState) => ({
                    searchBarVisible: true
                }));
            } else {
                this.setState((pre) => ({
                    searchBarVisible: false
                }));
            }
            this.render();
        }
    }


    render() {
        return (
            <>
                <Layout className={"layout"}>
                    <div className={"side-tool-bar"}>
                        <ul>
                            <li>
                                <Avatar size={40}
                                        src={"https://ryu2u-1305537946.cos.ap-nanjing.myqcloud.com/pictures%2FQQ%E5%9B%BE%E7%89%8720231118112223.jpg"}
                                />
                            </li>
                        </ul>
                    </div>

                    <Sider onWheel={e => this.sideMouseUpEvent(e)} id={"side-bar"} theme={"light"}>
                        {
                            this.state['searchBarVisible'] &&
                            <div className={"room-list-bar"}>
                                <Input prefix={<SearchOutlined/>}/>
                            </div>
                        }
                        <List className={"room-list"} itemLayout={"horizontal"}>
                            {this.roomList}
                        </List>
                    </Sider>
                    <Layout>
                        <Header className={"room-header"}></Header>
                        <Content id={"chatContent"}>
                            <div className="more-msg"></div>
                            <div className="message chat_right">
                                <div className="chat_right_content">
                                    <Avatar
                                        className="right_avatar"
                                        src={"https://ryu2u-1305537946.cos.ap-nanjing.myqcloud.com/pictures%2FQQ%E5%9B%BE%E7%89%8720231118112223.jpg"}
                                        gap={3}
                                        size={"large"}
                                    />
                                    <div className="chat_right_time">
                                        2023-10-11 12:12:13
                                    </div>
                                    <div className="chat_right_msg">
                                        <div>
                                            <p>
                                                Message .... Message ....
                                                Message .... Message ....
                                                Message .... Message ....
                                                Message .... Message ....
                                                Message .... Message ....
                                                Message .... Message ....
                                            </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div className={"message chat_left"}>
                                <div className={"chat_left_content"}>
                                    <Avatar className={"left_avatar"}
                                            gap={3}
                                            size={"large"}>
                                        Tauri
                                    </Avatar>
                                    <div className={"chat_left_time"}>
                                        2023-10-11 12:12:13
                                    </div>
                                    <div className={"chat_left_msg"}>
                                        <div>
                                            <p>
                                                Message .... Message ....
                                                Message .... Message ....
                                                Message .... Message ....
                                                Message .... Message ....
                                                Message .... Message ....
                                                Message .... Message ....
                                            </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                            <p> Message .... Message .... </p>
                                        </div>
                                    </div>
                                </div>
                            </div>

                        </Content>
                        <Footer id={"footer"}>
                            <div>
                                <p>footer</p>
                                <p>footer</p>
                                <p>footer</p>
                                <p>footer</p>
                                <p>footer</p>
                                <p>footer</p>
                                <p>footer</p>
                                <p>footer</p>
                            </div>

                        </Footer>
                    </Layout>
                </Layout>
            </>
        );
    }
}