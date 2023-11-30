import "./RoomComponent.scss"
import {Content, Footer, Header} from "antd/es/layout/layout";
import {Avatar, Button, Spin} from "antd";
import {Resizable} from "re-resizable";
import VditorEdit from "../../components/VditorEdit.tsx";
import {useEffect, useRef, useState} from "react";
import Vditor from "vditor";
import {emit, listen} from "@tauri-apps/api/event";
import {useAsyncError, useParams} from "react-router";
import {ChatMessage, ChatRoom, ProtoAckMessage, ProtoChatMessage, R, User} from "../../entity/Entity.ts";
import {invoke} from "@tauri-apps/api";
import {render} from "react-dom";
import {log} from "vditor/dist/ts/util/log";
import {uuid} from "../../common/constant";
import {Loading3QuartersOutlined, LoadingOutlined} from "@ant-design/icons";


export function RoomComponent() {
    const [vditor, setVditor] = useState<Vditor>();
    const [room, setRoom] = useState(new ChatRoom());
    const param = useParams();
    const [loading, setLoading] = useState(true);
    const [messageList, setMessageList] = useState<ChatMessage[]>([]);

    const msgList = useRef<ChatMessage[]>([]);
    const [currentUserId, setCurrentUserId] = useState(-1);
    const [latestTime, setLatestTime] = useState(0);
    const [hasMore, setHasMore] = useState(true);
    const [lastScrollHeight, setLastScrollHeight] = useState(0);
    const [chatContent, setChatContent] = useState(null);
    const [isBottom, setIsBottom] = useState(true);


    useEffect(() => {
        scrollToBottom(true);
    }, [chatContent]);
    useEffect(() => {
        if (chatContent) {
            console.log(`scrollTop : ${chatContent.scrollTop}`)
            console.log(`scrollHeight : ${chatContent.scrollHeight}`)
            console.log(`clientHeight :${chatContent.clientHeight}`)
            console.log("----");
        }
        if (isBottom) {
            scrollToBottom(false);
        } else {
            scrollOldHeight();
        }
    }, [messageList]);

    useEffect(() => {
        setLoading(true);
        setHasMore(true);
        setLastScrollHeight(0);
        invoke('get_room_info', {
            roomId: param['id']
        }).then((res: R) => {
            setRoom(res.data);
        });

        invoke('get_sys_time', {}).then((res: R) => {
            if (res.code == 200) {
                let sendTime: number = res.data;
                invoke('room_msg_list', {
                    roomId: param['id'],
                    sendTime: sendTime.toString()
                }).then((res: R) => {
                    if (res.code == 200) {
                        if (res.data.length > 0) {
                            msgList.current = res.data;
                            setMessageList([...msgList.current]);
                            setLoading(false);
                            setLatestTime(res.data[0].sendTime);
                            if (res.data.length < 10) {
                                setHasMore(false);
                            }
                        } else {
                            setHasMore(false);
                        }
                    }
                })
            }
        });

        invoke('get_user_info', {}).then((res: R) => {
            if (res.code == 200) {
                setCurrentUserId(res.data.id);
            }
        });

    }, [param]);


    useEffect(() => {
        const unlisten = listen('msg_read', event => {
            console.log("msg_read => ")
            let chatMsg: ProtoChatMessage = event.payload;
            let newMsg: ChatMessage = new ChatMessage();
            newMsg.id = chatMsg.id;
            newMsg.content = chatMsg.content;
            newMsg.roomId = chatMsg.chat_room_id;
            newMsg.sendTime = chatMsg.send_time;
            newMsg.senderId = chatMsg.sender_id;
            newMsg.senderAvatar = "https://ryu2u-1305537946.cos.ap-nanjing.myqcloud.com/pictures/f8f7390913fc461efc15497dfeeb0cc6.png";
            addNewMsg(newMsg);
        });

        const unlisten_ack = listen('msg_ack', event => {
            let ackMsg: ProtoAckMessage = event.payload;
            if (ackMsg.code == 200) {
                const msg_id = ackMsg.msg_id;
                msgList.current.forEach(v => {
                    if (v.id == msg_id) {
                        v.isSend = true;
                    }
                });
                setMessageList([...msgList.current]);
            }
        });

        return () => {
            unlisten.then(f => f());
            unlisten_ack.then(f => f());
        }
    }, []);

    function addNewMsg(newMsg: ChatMessage) {
        let arr = msgList.current.filter(v => v.id == newMsg.id);
        if (arr.length <= 0) {
            console.log("before -> ")
            console.log(msgList.current)
            console.log(newMsg.content);
            msgList.current.push(newMsg);
            console.log("after -> ")
            console.log(msgList.current);
            setMessageList([]);
            setMessageList([...msgList.current]);
        }
    }

    function getMessageList(sendTime: string) {
        invoke('room_msg_list', {
            roomId: param['id'],
            sendTime: sendTime
        }).then((res: R) => {
            if (res.code == 200) {
                console.log(res.data);
                if (res.data.length > 0) {
                    let arr: ChatMessage[] = res.data;
                    for (let i = arr.length - 1; i >= 0; i--) {
                        msgList.current.unshift(arr[i]);
                    }
                    console.log(msgList.current);
                    setMessageList([]);
                    setMessageList([...msgList.current]);
                    setLatestTime(res.data[0].sendTime);
                    scrollOldHeight();

                    if (arr.length < 10) {
                        setHasMore(false);
                    }
                } else {
                    setHasMore(false);
                }
                // console.log(messageList);
                console.log("latestTime : " + latestTime);
            }
        })
    }

    function sendMsg() {
        const html = vditor!.getValue()!;
        if (!html || html == "") {
            return;
        }
        console.log("sen_msg -->");
        console.log(html);
        let user: User = JSON.parse(localStorage.getItem("user_info"));
        let msg: ChatMessage = new ChatMessage();
        msg.id = uuid();
        msg.roomId = room.id;
        msg.content = html;
        msg.senderId = user.id;
        msg.senderAvatar = user.avatarPath;
        emit('group_msg_send', msg).then(v => {
            vditor!.setValue("");
            msg.isSend = false;
            msgList.current.push(msg);
            setMessageList([...msgList.current]);
            scrollToBottom(true);
        });
    }

    function getMoreMsg(event: Event) {
        console.log("more");
        getMessageList(latestTime.toString());
    }


    /**
     * 滚动到最底部
     * 默认是当前页面在最底部时自动滚动
     *
     * @param refresh 默认为false ，表示只有在最底部才滚动，true 为强制滚动
     */
    function scrollToBottom(refresh: boolean) {
        if (!refresh && chatContent && chatContent.clientHeight + chatContent.scrollTop > (chatContent.scrollHeight - 80)) {
            setIsBottom(true);
            console.log("自动滚动!");
            chatContent.scrollTop = chatContent.scrollHeight;
            setLastScrollHeight(chatContent.scrollHeight);
        } else if (refresh && chatContent) {
            console.log("强制滚动")
            chatContent.scrollTop = chatContent.scrollHeight;
            setLastScrollHeight(chatContent.scrollHeight);
        }
    }

    function scrollOldHeight() {
        console.log(`scrollTop : ${chatContent.scrollTop}`)
        console.log(`scrollHeight : ${chatContent.scrollHeight}`)
        console.log(`clientHeight :${chatContent.clientHeight}`)
        chatContent.scrollTop = 20;
        setLastScrollHeight(chatContent.scrollHeight);
    }


    return (
        <>
            {
                loading ?
                    <div className={"loading-div"}>
                        <Spin size={'large'}/>
                    </div>
                    :
                    <>
                        <Header className={"room-header"}>
                            <p className={"room-title"}>
                                {room ? room.roomName : ''}
                            </p>
                        </Header>
                        <Content id={"chatContent"} ref={setChatContent}>
                            <div className="more-msg">
                                {
                                    hasMore ?
                                        <a onClick={(e) => getMoreMsg(e)}>
                                            显示更多消息
                                        </a>
                                        :
                                        <span>
                                        没有更多消息
                                        </span>
                                }
                            </div>
                            {
                                messageList.map((msg: ChatMessage) => (
                                    <div key={msg.id}
                                         className={msg.senderId == currentUserId ? "message chat_right" : "message chat_left"}>
                                        <div className={"chat_left_content"}>
                                            <Avatar
                                                className={msg.senderId == currentUserId ? "right_avatar" : "left_avatar"}
                                                src={msg.senderAvatar}
                                                gap={3}
                                                size={"large"}
                                            />
                                            <div
                                                className={msg.senderId == currentUserId ? "chat_right_time" : "chat_left_time"}>
                                                {msg.sendTime % 10 == 0 ? new Date(msg.sendTime).toLocaleString('chinese', {hour12: false}) :
                                                    <span>&nbsp;</span>}
                                            </div>
                                            <div className={msg.senderId == currentUserId ? "chat_right_msg" : "chat_left_msg"}>
                                                <div>
                                                    {msg.content}
                                                </div>
                                            </div>
                                            {
                                                msg.senderId == currentUserId &&
                                                !msg.isSend &&
                                                <div className={"spin-loading"}>
                                                    <Loading3QuartersOutlined spin={true}/>
                                                </div>
                                            }
                                        </div>
                                    </div>
                                ))
                            }
                        </Content>
                        <Resizable
                            handleClasses={{top: 'resize-div'}}
                            minHeight={201}
                            maxHeight={"70%"}
                            enable={{
                                top: true,
                                right: false,
                                bottom: false,
                                left: false,
                                topRight: false,
                                bottomRight: false,
                                bottomLeft: false,
                                topLeft: false
                            }}
                        >
                            <Footer id={"footer"}>
                                <VditorEdit getVditor={(e) => setVditor(e)}/>
                                <div className={"editor-footer"}>
                                    <Button type={"primary"} onClick={sendMsg}>
                                        发送
                                    </Button>
                                </div>
                            </Footer>
                        </Resizable>
                    </>
            }


        </>
    );
}