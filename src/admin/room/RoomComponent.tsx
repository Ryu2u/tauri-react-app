import "./RoomComponent.scss"
import {Content, Footer, Header} from "antd/es/layout/layout";
import {Avatar, Button,  Spin} from "antd";
import {Resizable} from "re-resizable";
import VditorEdit from "../../components/VditorEdit.tsx";
import {useEffect, useState} from "react";
import Vditor from "vditor";
import {emit} from "@tauri-apps/api/event";
import {useParams} from "react-router";
import {ChatRoom, R} from "../../entity/Entity.ts";
import {invoke} from "@tauri-apps/api";


export function RoomComponent() {

    const [vditor, setVditor] = useState<Vditor>();
    const [room, setRoom] = useState(new ChatRoom());
    const param = useParams();
    const [loading,setLoading] = useState(true);

    useEffect(() => {
        setLoading(true);
        invoke('get_room_info', {
            roomId: param['id']
        }).then((res: R) => {
            setRoom(res.data);
            setLoading(false);
        });
    }, [param]);

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


    return (
        <>
            {
                loading?
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