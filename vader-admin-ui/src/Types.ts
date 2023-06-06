
type EventType = "UserEvent" | { TeamEvent: { team_size: number } };
interface EventInfo {
  id: string;
  name: string;
  logo: string;
  event_type: EventType
}


interface PlayerInfo {
  id: string;
  name: string;
  logo: string;
  score: number;
}

type UserInfo = PlayerInfo;
type TeamInfo = PlayerInfo;
