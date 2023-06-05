import { useEffect, useState } from "react";

const Event = () => {
    const [curEvent, setCurEvent] = useState<EventInfo|null>(null);
    useEffect(() => {
        (async () => {
            const url = "http://localhost:8080/event/info";
            const res = await fetch(url, { method: "GET" });
            const event:EventInfo=await res.json();
            setCurEvent
        })();
    });
};

export default Event;
