import { useEffect, useState } from "react";
import EventFactory from "../components/EventFactory";
import { useNavigate } from "react-router-dom";

const Event = () => {
    const [curEvent, setCurEvent] = useState<EventInfo>();
    const navigate = useNavigate();
    useEffect(() => {
        (async () => {
            const url = "http://localhost:8080/event/info";
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
