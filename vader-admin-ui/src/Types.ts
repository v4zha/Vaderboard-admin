export type EventType = "UserEvent" | { TeamEvent: { team_size: number } };

export interface EventInfo {
  id: string;
  name: string;
  logo: string;
  event_type: EventType;
  state: EventState;
}
export enum EventState {
  Added,
  Start,
  Stop,
}



export interface PlayerInfo {
  id: string;
  name: string;
  logo: string;
  score: number;
}


export interface ScoreUpdate {
  id: string;
  score: number;
}
export type UserInfo = PlayerInfo;
export type TeamInfo = PlayerInfo;
