interface FactoryProps {
    eventInfo: EventInfo;
}
const EventFactory: React.FC<FactoryProps> = (
    props: FactoryProps
): JSX.Element => {
    console.log(props);
    return <></>;
};
export default EventFactory;
