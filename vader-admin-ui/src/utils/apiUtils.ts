export const getEvents = async (): Promise<Array<EventInfo>> => {
    let events: Array<EventInfo> = [];
    const url = "http://localhost:8080/event/info/all";
    const res = await fetch(url, { method: "GET" });
    events = await res.json();
    return events;
}
export const getUsers = async (): Promise<Array<UserInfo>> => {
    let users: Array<UserInfo> = [];
    const url = "http://localhost:8080/user/info/all";
    const res = await fetch(url, { method: "GET" });
    users = await res.json();
    return users;
}

export const getTeams = async (): Promise<Array<TeamInfo>> => {
    let teams: Array<TeamInfo> = [];
    const url = "http://localhost:8080/team/info/all";
    const res = await fetch(url, { method: "GET" });
    teams = await res.json();
    return teams;
}

