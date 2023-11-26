export class R {
    code?: number;
    msg?: string;
    data?: any;
}

export class User {


}

export class ChatRoom {
    id!: number;
    isGroup!: boolean;
    roomName!: string;
    roomAvatar!: string;
    isTop!: boolean;
    isView!: boolean;
}

export class  ChatMessage{
    id!:string;
    roomId!:number;
    content!:string;
    senderId!:number;
    senderName!:string;
    sendTime!:number;
    createdBy!:number;
    createdTime!:number;
}