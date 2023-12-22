import  {useEffect, useRef, useState} from "react";
import {Avatar, Badge, Input, Layout, List} from "antd";
import Sider from "antd/es/layout/Sider";
import "./ChatComponent.scss"
import {SearchOutlined} from "@ant-design/icons";
import {listen} from "@tauri-apps/api/event";
import {Outlet, useNavigate} from "react-router";
import {invoke} from "@tauri-apps/api";
import {ChatMessage, ChatRoom, ProtoAckMessage, ProtoChatMessage, R} from "../../entity/Entity.ts";
import {Resizable} from "re-resizable";
import {USER_AVATAR_PATH} from "../../common/constant";

export function ChatComponent() {

    const [searchBarVisible, setSearchBarVisible] = useState(false);
    const navigate = useNavigate();
    const chatRoomListRef = useRef<ChatRoom[]>([]);
    const [roomList, setRoomList] = useState<ChatRoom[]>([]);
    const [currentRoomId, setCurrentRoomId] = useState(-1);
    const [avatar, setAvatar] = useState('');

    useEffect(() => {
        console.log("chatComponent : " + window.location.toString());
        let user_avatar = localStorage.getItem(USER_AVATAR_PATH);
        if (user_avatar) {
            setAvatar(user_avatar);
        }

        const url: string = window.location.toString();
        if (url.includes("/chat/room/")) {
            const i = url.lastIndexOf("/");
            if (i != -1) {
                const id = url.substring(i + 1);
                setCurrentRoomId(parseInt(id));
            }
        }

        invoke('get_chat_room_list', {}).then((res: R) => {
            console.log(res);
            setRoomList(res.data);
            chatRoomListRef.current = res.data;
            console.log("roomList -> ")
            console.log(roomList);
        });

    }, []);

    useEffect(() => {
        console.log("current Room -> " + currentRoomId);
        const unlisten = listen('msg_read', event => {
            console.log("msg_read => ")
            let chatMsg: ProtoChatMessage | unknown = event.payload;
            let list = [...chatRoomListRef.current];
            list.forEach(r => {
                if (r.id == chatMsg.chat_room_id) {
                    r.latestMsg = chatMsg.content;
                    if (chatMsg.chat_room_id == currentRoomId) {
                        return;
                    }
                    r.unreadCount++;
                }
            });
            setRoomList(list);
        });

        const unlisten_ack = listen('msg_ack', event => {
            let ackMsg: ProtoAckMessage | unknown = event.payload;
            let list = [...chatRoomListRef.current];
            list.forEach(r => {
                if (r.id == ackMsg.room_id) {
                    r.unreadCount--;
                }
            });
            setRoomList(list);
        });

        return () => {
            unlisten.then(f => f());
            unlisten_ack.then(f => f());
        }
    }, [currentRoomId]);


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

    function setLatestMsg(msg: string): string {
        const span = document.createElement("span");
        span.innerHTML = msg;
        return span.innerText;
    }

    return (
        <>
            <Layout className={"layout"}>
                <div className={"side-tool-bar"}>
                    <ul>
                        <li>
                            <Avatar size={40}
                                    src={avatar}
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
                                        description={<p className={"room-title"}>{room.latestMsg ? setLatestMsg(room.latestMsg) : <span>&nbsp;</span>}</p>}
                                    >
                                    </List.Item.Meta>
                                    <div>
                                        <Badge count={room.unreadCount} offset={[-5, 25]}/>
                                    </div>
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