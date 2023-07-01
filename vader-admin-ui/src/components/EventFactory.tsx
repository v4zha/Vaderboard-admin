import UserEvent from "./UserEvent";
import TeamEvent from "./TeamEvent";
import { EventInfo } from "../Types";

interface FactoryProps {
    eventInfo: EventInfo;
}
const EventFactory: React.FC<FactoryProps> = (
    {eventInfo}: FactoryProps
): JSX.Element => {
    return eventInfo.event_type === "UserEvent" ? (
            <UserEvent eventInfo={eventInfo} />
        ) : (
            <TeamEvent eventInfo={eventInfo} />
        );
};
export default EventFactory;
