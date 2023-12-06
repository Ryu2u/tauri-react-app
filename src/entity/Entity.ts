export class R {
    code?: number;
    msg?: string;
    data?: any;
}

export class User {
    id!: number;
    username!: string;
    nickname!: string;
    avatarPath!: string;
    createdBy!: number;
    createdTime!: number;
}

export class ChatRoom {
    id!: number;
    isGroup!: boolean;
    roomName!: string;
    roomAvatar!: string;
    isTop!: boolean;
    isView!: boolean;
    unreadCount: number = 100;
    latestMsg:string;
}

export class ChatMessage {
    id!: string;
    roomId!: number;
    content!: string;
    senderId!: number;
    senderName!: string;
    senderAvatar!: string;
    sendTime!: number;
    createdBy!: number;
    createdTime!: number;
    isSend: boolean = true;
}

export class ProtoChatMessage {
    id!: string;
    chat_room_id!: number;
    chat_room_name!: string;
    content!: string;
    sender_id!: number;
    sender_name!: string;
    sender_avatar!: string;
    receiver_id!: number;
    receiver_count!: number;
    read_count!: number;
    is_read!: boolean;
    send_time!: number;
}

export class ProtoAckMessage {
    code!: number;
    msg_content!: string;
    msg_id!: string;
    room_id!: number;
    user_id!: number;
    msg_type!: number;
}