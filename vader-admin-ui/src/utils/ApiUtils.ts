import { EventInfo, TeamInfo, UserInfo } from "../Types";


const API_URL=import.meta.env.VITE_API_URL;
export const apiUrl = "http://{API_URL}";
export const apiUrlWs = "ws://{API_URL}";

export const getEvents = async (): Promise<Array<EventInfo>> => {
  let events: Array<EventInfo> = [];
  const url = `${apiUrl}/event/info/all`;
  const res = await fetch(url, { method: "GET" });
  events = await res.json();
  return events;
}
export const getUsers = async (): Promise<Array<UserInfo>> => {
  let users: Array<UserInfo> = [];
  const url = `${apiUrl}/user/info/all`;
  const res = await fetch(url, { method: "GET" });
  users = await res.json();
  return users;
}

export const getTeams = async (): Promise<Array<TeamInfo>> => {
  let teams: Array<TeamInfo> = [];
  const url = `${apiUrl}/team/info/all`;
  const res = await fetch(url, { method: "GET" });
  teams = await res.json();
  return teams;
}


export const eventFtsUrl = `${apiUrlWs}/event/fts/20`;

export const teamFtsUrl = `${apiUrlWs}/team/fts/20`;

export const userFtsUrl = `${apiUrlWs}/user/fts/20`;

export const userCurFtsUrl = `${apiUrlWs}/event/info/user/20`;
export const teamCurFtsUrl = `${apiUrlWs}/event/info/team/20`;
export const remMemCurFtsUrl = `${apiUrlWs}/event/info/team/rem_members/20`;
