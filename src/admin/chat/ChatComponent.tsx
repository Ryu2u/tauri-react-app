import {Component, useState} from "react";
import {Avatar, Input, Layout, List} from "antd";
import Sider from "antd/es/layout/Sider";
import {Content, Footer, Header} from "antd/es/layout/layout";
import "./chat.scss"
import {SearchOutlined} from "@ant-design/icons";
import {Resizable} from "re-resizable";
import TextArea from "antd/es/input/TextArea";

export function ChatComponent() {

    const [searchBarVisible, setSearchBarVisible] = useState(false);

    const list: number[] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    const roomList = list.map(v =>
        <List.Item key={v} className={"room-list-item"}>
            <List.Item.Meta
                avatar={<Avatar src={"https://ryu2u-1305537946.cos.ap-nanjing.myqcloud.com/pictures%2FQQ%E5%9B%BE%E7%89%8720231118112223.jpg"}/>}
                title={"Hello World "}
                description={v}
            >
            </List.Item.Meta>
        </List.Item>
    );

    function sideMouseUpEvent(event: React.WheelEvent<HTMLDivElement>) {
        const sidebar = document.getElementById("side-bar")!;
        if (sidebar) {
            if (sidebar.scrollTop == 0) {
                setSearchBarVisible(true);
            } else {
                setSearchBarVisible(false);
            }
        }
    }

    return (
        <>
            <Layout className={"layout"}>
                <Sider onWheel={e => sideMouseUpEvent(e)} id={"side-bar"} theme={"light"}>
                    {
                        searchBarVisible &&
                        <div className={"room-list-bar"}>
                            <Input prefix={<SearchOutlined/>}/>
                        </div>
                    }
                    <List className={"room-list"} itemLayout={"horizontal"}>
                        {roomList}
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
                    <Resizable
                        defaultSize={{
                            height: 200
                        }}
                        handleClasses={{top: 'resize-div'}}
                        minHeight={300}
                        maxHeight={"70%"}
                        enable={{top: true, right: false, bottom: false, left: false, topRight: false, bottomRight: false, bottomLeft: false, topLeft: false}}
                    >
                        <Footer id={"footer"}>
                            <TextArea classNames={"input-area"}/>
                        </Footer>
                    </Resizable>
                </Layout>
            </Layout>
        </>
    );
}