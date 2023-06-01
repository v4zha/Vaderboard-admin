interface EventInfo {
    id: string;
    name: string;
    logo: string;
    event_type: EventType;
}
enum EventType {
    TeamEvent = "TeamEvent",
    UserEvent = "UserEvent"
}

interface PlayerInfo {
    id: string;
    name: string;
    logo: string;
    score: number;
}

type UserInfo = PlayerInfo;
type TeamInfo = PlayerInfo;
