import "./RoomComponent.scss"
import {Content, Footer, Header} from "antd/es/layout/layout";
import {Avatar, Button, Spin} from "antd";
import {Resizable} from "re-resizable";
import VditorEdit from "../../components/VditorEdit.tsx";
import {useEffect, useState} from "react";
import Vditor from "vditor";
import {emit} from "@tauri-apps/api/event";
import {useParams} from "react-router";
import {ChatMessage, ChatRoom, R} from "../../entity/Entity.ts";
import {invoke} from "@tauri-apps/api";


export function RoomComponent() {

    const [vditor, setVditor] = useState<Vditor>();
    const [room, setRoom] = useState(new ChatRoom());
    const param = useParams();
    const [loading, setLoading] = useState(true);
    const [messageList, setMessageList] = useState([]);
    const [currentUserId, setCurrentUserId] = useState(-1);
    const [latestTime, setLatestTime] = useState(0);
    const [hasMore, setHasMore] = useState(true);
    const [lastScrollHeight, setLastScrollHeight] = useState(0);
    const [chatContent, setChatContent] = useState(null);

    useEffect(() => {

        scrollToBottom(true);

    }, [chatContent])

    useEffect(() => {
        setLoading(true);
        setHasMore(true);

        invoke('get_room_info', {
            roomId: param['id']
        }).then((res: R) => {
            setRoom(res.data);
            setLoading(false);
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
                            setMessageList(res.data);
                            setLatestTime(res.data[0].sendTime);
                            scrollOldHeight();
                            scrollToBottom(false);
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
                        messageList.unshift(arr[i]);
                    }
                    setMessageList(messageList);
                    setLatestTime(res.data[0].sendTime);
                    scrollOldHeight();
                    scrollToBottom(false);
                    if (arr.length < 10) {
                        setHasMore(false);
                    }
                } else {
                    setHasMore(false);
                }
                console.log(messageList);
                console.log("latestTime : " + latestTime);
            }
        })
    }

    function getParam() {
        console.log("room id : ");
        console.log(param);
    }

    getParam();

    function sendMsg() {
        const html = vditor!.getHTML()!;
        if (!html || html == "") {
            return;
        }
        console.log("sen_msg -->");
        console.log(html);
        emit('msg_send', {
            msg: html
        }).then(v => {
            vditor!.setValue("");
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
        console.log("chatContent")
        console.log(chatContent);
        if (!refresh && chatContent && chatContent.scrollHeight - chatContent.scrollTop == chatContent.clientHeight) {
            console.log("自动滚动!");
            chatContent.scrollTop = chatContent.scrollHeight;
            setLastScrollHeight(chatContent.scrollHeight);
        } else if (refresh && chatContent) {
            console.log("强制滚动")
            chatContent.scrollTop = chatContent.scrollHeight;
            setLastScrollHeight(chatContent.scrollHeight);
        }
        console.log("last height : " + lastScrollHeight);
    }

    function scrollOldHeight() {
        console.log("last height : " + lastScrollHeight);
        console.log("scrollTop : ", chatContent.scrollTop);
        console.log("scrollHeight : ", chatContent.scrollHeight);
        let height = chatContent.scrollHeight - lastScrollHeight;
        console.log(height);
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
                                    <div key={msg.id} className={msg.senderId == currentUserId ? "message chat_right" : "message chat_left"}>
                                        <div className={msg.senderId == currentUserId ? "chat_right_content" : "chat_left_content"}>
                                            <Avatar
                                                className={msg.senderId == currentUserId ? "right_avatar" : "left_avatar"}
                                                src={msg.senderAvatar}
                                                gap={3}
                                                size={"large"}
                                            />
                                            <div className={msg.senderId == currentUserId ? "chat_right_time" : "chat_left_time"}>
                                                {msg.sendTime % 10 == 0 ? new Date(msg.sendTime).toLocaleString('chinese', {hour12: false}) : <span>&nbsp;</span>}
                                            </div>
                                            <div className={msg.senderId == currentUserId ? "chat_right_msg" : "chat_left_msg"}>
                                                <div>
                                                    {msg.content}
                                                </div>
                                            </div>
                                        </div>
                                    </div>

                                ))
                            }


                        </Content>
                        <Resizable
                            handleClasses={{top: 'resize-div'}}
                            minHeight={201}
                            maxHeight={"70%"}
                            enable={{top: true, right: false, bottom: false, left: false, topRight: false, bottomRight: false, bottomLeft: false, topLeft: false}}
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