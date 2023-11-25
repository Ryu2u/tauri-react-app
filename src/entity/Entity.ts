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