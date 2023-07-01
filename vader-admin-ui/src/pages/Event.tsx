import { useEffect, useState } from "react";
import EventFactory from "../components/EventFactory";
import { useNavigate } from "react-router-dom";
import { EventInfo } from "../Types";
import { apiUrl } from "../utils/apiUtils";

const Event = () => {
    const [curEvent, setCurEvent] = useState<EventInfo>();
    const navigate = useNavigate();
    useEffect(() => {
        (async () => {
            const url = `${apiUrl}/event/info`;
            const res = await fetch(url, { method: "GET" });
            if (res.ok) {
                const event: EventInfo = await res.json();
                setCurEvent(event);
            } else {
                navigate("/event/add");
            }
        })();
    }, []);
    return curEvent ? <EventFactory eventInfo={curEvent} /> : <></>;
};

export default Event;
