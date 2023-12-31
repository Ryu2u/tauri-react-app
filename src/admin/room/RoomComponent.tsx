import "./RoomComponent.scss"
import {Content, Footer, Header} from "antd/es/layout/layout";
import {Avatar, Button, Spin} from "antd";
import {Resizable} from "re-resizable";
import VditorEdit from "../../components/VditorEdit.tsx";
import {useEffect, useRef, useState} from "react";
import {emit, listen} from "@tauri-apps/api/event";
import {useAsyncError, useParams} from "react-router";
import {ChatMessage, ChatRoom, ProtoAckMessage, ProtoChatMessage, ProtoResponseMessage, R, User} from "../../entity/Entity.ts";
import {invoke} from "@tauri-apps/api";
import {USER_AVATAR_PATH, USER_KEY, uuid} from "../../common/constant";
import {Loading3QuartersOutlined} from "@ant-design/icons";
import Vditor from "vditor";

export function RoomComponent() {
    const [vditor, setVditor] = useState<Vditor>();
    const [room, setRoom] = useState(new ChatRoom());
    const param = useParams();
    const [loading, setLoading] = useState(true);
    const [messageList, setMessageList] = useState<ChatMessage[]>([]);
    const [getMsgLoading, setGetMsgLoading] = useState(false);
    const [currentRoomId, setCurrentRoomId] = useState(null);

    const msgList = useRef<ChatMessage[]>([]);
    const [currentUserId, setCurrentUserId] = useState(-1);
    const [latestTime, setLatestTime] = useState(0);
    const [hasMore, setHasMore] = useState(true);
    const [lastScrollHeight, setLastScrollHeight] = useState(0);
    const [chatContent, setChatContent] = useState(null);
    const [isBottom, setIsBottom] = useState(true);


    useEffect(() => {
        console.log("init ")
        scrollToBottom(true);
    }, [chatContent]);

    useEffect(() => {
        if (chatContent) {
            // console.log(`scrollTop : ${chatContent.scrollTop}`)
            // console.log(`scrollHeight : ${chatContent.scrollHeight}`)
            // console.log(`clientHeight :${chatContent.clientHeight}`)
            // console.log("----");
        }
        if (isBottom) {
            scrollToBottom(false);
        } else {
            scrollOldHeight();
        }
    }, [messageList]);

    useEffect(() => {
        if (currentRoomId != param['id']) {
            console.log("set id");
            setCurrentRoomId(param['id']);
        }
    }, [param]);


    useEffect(() => {
        console.log("currentRoomId room -> " + currentRoomId);
        let user_id: string = localStorage.getItem(USER_KEY);
        setCurrentUserId(parseInt(user_id));
        setLoading(true);
        setHasMore(true);
        setMessageList([]);
        msgList.current = [];
        setLastScrollHeight(0);
        invoke('get_room_info', {
            roomId: param['id']
        }).then((res: R) => {
            setRoom(res.data);
        });

        invoke('get_sys_time', {}).then((res: R) => {
            if (res.code == 200) {
                let sendTime: number = res.data;
                setMessageList([...msgList.current]);
                getMessageList(sendTime.toString());
            }
        });


    }, [currentRoomId]);


    useEffect(() => {
        console.log("room -> url");
        console.log(window.location.toString());
        const unlisten = listen('msg_read', event => {
            console.log("msg_read => ")
            console.log(typeof event.payload)
            console.log(event.payload)
            let chatMsg: ProtoChatMessage | unknown = event.payload;
            let newMsg: ChatMessage = new ChatMessage();
            console.log(`newMsg.roomId ${chatMsg.chat_room_id}`);
            if (chatMsg.chat_room_id.toString() == param['id']) {
                newMsg.id = chatMsg.id;
                newMsg.content = chatMsg.content;
                newMsg.roomId = chatMsg.chat_room_id;
                newMsg.sendTime = chatMsg.send_time;
                newMsg.senderId = chatMsg.sender_id;
                newMsg.senderAvatar = chatMsg.sender_avatar;
                addNewMsg(newMsg);
            }
        });

        const unlisten_resp = listen('msg_response', event => {
            let respMsg: ProtoResponseMessage | unknown = event.payload;
            if (respMsg.code == 200) {
                const msg_id = respMsg.msg_id;
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
            unlisten_resp.then(f => f());
        }
    }, [currentRoomId]);

    function addNewMsg(newMsg: ChatMessage) {
        console.log("before -> ")
        console.log(msgList.current)
        console.log(newMsg.content);
        msgList.current.push(newMsg);
        console.log("after -> ")
        console.log(msgList.current);
        setMessageList([...msgList.current]);
    }

    function getMessageList(sendTime: string) {
        invoke('room_msg_list', {
            roomId: param['id'],
            sendTime: sendTime
        }).then((res: R) => {
            if (res.code == 200) {
                if (res.data.length < 10) {
                    setHasMore(false);
                }
                if (res.data.length > 0) {
                    let arr: ChatMessage[] = res.data;
                    arr = arr.filter(v => {
                        for (let chatMessage of msgList.current) {
                            if (chatMessage.id == v.id) {
                                return false;
                            }
                        }
                        return true;
                    });
                    for (let i = arr.length - 1; i >= 0; i--) {
                        arr[i].isSend = true;
                        msgList.current.unshift(arr[i]);
                        // todo 判断当前消息是已读状态还是未读状态，若为已读状态，则发送已读消息，同时减少当前聊天室的未读消息数量
                        const user_id: string = localStorage.getItem(USER_KEY);
                        const userId = parseInt(user_id);
                        if (arr[i].senderId != userId && !arr[i].isRead) {
                            console.log("current user id --> " + currentUserId);
                            console.log("未读消息 --> " + arr[i].id);
                            emit('ack_msg_send', arr[i]).then()
                        }
                    }
                    console.log("list -> ")
                    console.log(msgList.current);
                    setMessageList([...msgList.current]);
                    setLatestTime(res.data[0].sendTime);
                    scrollOldHeight();
                }
                setGetMsgLoading(false);
                setLoading(false);
            }
        })
    }

    function sendMsg() {
        let html: string = vditor!.getHTML()!;
        if (!html || html == "") {
            return;
        }
        html = html.substring(0, html.lastIndexOf("\n"));
        console.log("sen_msg -->");
        console.log(html);
        let user_id = parseInt(localStorage.getItem(USER_KEY));
        let user_avatar = localStorage.getItem(USER_AVATAR_PATH);
        let msg: ChatMessage = new ChatMessage();
        msg.id = uuid();
        msg.roomId = room.id;
        msg.content = html;
        msg.senderId = user_id;
        msg.senderAvatar = user_avatar;
        emit('group_msg_send', msg).then(v => {
            vditor!.setValue("");
            msg.isSend = false;
            msgList.current.push(msg);
            setMessageList([...msgList.current]);
            setTimeout(() => {
                scrollToBottom(true);
            }, 100);
        });
    }

    function getMoreMsg(event?: Event) {
        console.log("more");
        setGetMsgLoading(true);
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
        if (chatContent) {
            chatContent.scrollTop = 20;
            setLastScrollHeight(chatContent.scrollHeight);
        }
    }

    function chatContentWheel(event: React.WheelEvent) {
        if (chatContent && chatContent.scrollTop == 0 && !getMsgLoading && hasMore) {
            getMoreMsg();
        }
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
                        <Content id={"chatContent"} onWheel={e => chatContentWheel(e)} ref={setChatContent}>
                            <div className="more-msg">
                                {
                                    hasMore ?
                                        getMsgLoading ?
                                            <Loading3QuartersOutlined spin={true}/>
                                            :
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
                                                <span className={"msg_content"} dangerouslySetInnerHTML={{
                                                    __html: msg.content
                                                }}>
                                                    {/*{msg.content}*/}
                                                </span>
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