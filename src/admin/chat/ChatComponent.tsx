import React, {useEffect, useState} from "react";
import {Avatar, Input, Layout, List} from "antd";
import Sider from "antd/es/layout/Sider";
import "./ChatComponent.scss"
import {SearchOutlined} from "@ant-design/icons";
import {listen} from "@tauri-apps/api/event";
import {Outlet, useNavigate} from "react-router";
import {invoke} from "@tauri-apps/api";
import {ChatRoom, R} from "../../entity/Entity.ts";
import {Resizable} from "re-resizable";

export function ChatComponent() {

    const [searchBarVisible, setSearchBarVisible] = useState(false);
    const navigate = useNavigate();
    const [roomList, setRoomList] = useState([]);
    const [currentRoomId, setCurrentRoomId] = useState(-1);

    useEffect(() => {
        console.log("url");
        console.log(window.location.toString());
        const url: string = window.location.toString();
        if (url.includes("/chat/room/")) {
            const i = url.lastIndexOf("/");
            if (i != -1) {
                const id = url.substring(i + 1);
                setCurrentRoomId(parseInt(id));
            }
        }

        listen('msg_read', (event) => {
            // console.log("Get Msg --> ")
            // console.log(event.payload);
        }).then();

        invoke('get_chat_room_list', {}).then((res: R) => {
            console.log(res);
            setRoomList(res.data);
            console.log("roomList -> ")
            console.log(roomList);
        });

    }, []);


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

    function roomClick(id: number) {
        console.log(id);
        setCurrentRoomId(id);
        navigate(`/admin/chat/room/${id}`);
    }


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
                <Resizable
                    minWidth={220}
                    maxWidth={300}
                    handleClasses={{top: 'resize-div'}}
                    enable={{
                        top: false,
                        right: true,
                        bottom: false,
                        left: false,
                        topRight: false,
                        bottomRight: false,
                        bottomLeft: false,
                        topLeft: false
                    }}
                >
                    <Sider width={'auto'} onWheel={e => sideMouseUpEvent(e)} id={"side-bar"} theme={"light"}>
                        {
                            searchBarVisible &&
                            <div className={"room-list-bar"}>
                                <Input prefix={<SearchOutlined/>}/>
                            </div>
                        }
                        <List className={"room-list"} itemLayout={"horizontal"}>
                            {roomList.map((room: ChatRoom) => (
                                <List.Item key={room.id} onClick={() => roomClick(room.id)}
                                           className={currentRoomId == room.id ? 'room-list-item-checked' : 'room-list-item'}>
                                    <List.Item.Meta
                                        avatar={<Avatar src={room.roomAvatar}/>}
                                        title={<p className={"room-title"}>{room.roomName}</p>}
                                        description={"123"}
                                    >
                                    </List.Item.Meta>
                                </List.Item>
                            ))}
                        </List>
                    </Sider>
                </Resizable>

                <Layout>
                    <Outlet/>
                </Layout>
            </Layout>
        </>
    );
}