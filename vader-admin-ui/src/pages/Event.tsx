import { useEffect, useState } from "react";
import EventGen from "./AddEvent";
import EventFactory from "../components/EventFactory";

const Event = () => {
    const [curEvent, setCurEvent] = useState<EventInfo | null>(null);
    useEffect(() => {
        (async () => {
            const url = "http://localhost:8080/event/info";
            const res = await fetch(url, { method: "GET" });
            if (res.ok) {
                const event: EventInfo = await res.json();
                setCurEvent(event);
            }
        })();
    }, []);
    return curEvent ? <EventFactory eventInfo={curEvent} /> : <EventGen />;
};

export default Event;
