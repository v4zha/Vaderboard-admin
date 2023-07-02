import { useEffect, useState } from "react";
import { useNavigate, useLocation } from "react-router-dom";
import { EventInfo } from "../Types";
import { apiUrl } from "../utils/apiUtils";
import { UserEvent } from "../components/UserEvent";
import { TeamEvent } from "../components/TeamEvent";
import { UserEventOpts, TeamEventOpts } from "../Types";

export const VaderEvent = (): JSX.Element => {
    const [curEvent, setCurEvent] = useState<EventInfo | undefined>();
    const navigate = useNavigate();
    const location = useLocation();

    useEffect(() => {
        (async () => {
            const url = `${apiUrl}/event/info`;
            const res = await fetch(url, { method: "GET" });
            if (res.ok) {
                const event: EventInfo = await res.json();
                console.log("Getting cur event: ", event);
                setCurEvent(event);
            } else {
                navigate("/event/add");
            }
        })();
    }, []);

    return curEvent ? (
        curEvent.event_type === "UserEvent" ? (
            <UserEvent
                eventInfo={curEvent as EventInfo}
                drawerOpt={
                    (location.state?.opt as UserEventOpts) ??
                    UserEventOpts.EventDetails
                }
            />
        ) : (
            <TeamEvent
                eventInfo={curEvent as EventInfo}
                drawerOpt={
                    (location.state?.opt as TeamEventOpts) ??
                    TeamEventOpts.EventDetails
                }
            />
        )
    ) : (
        <></>
    );
};
