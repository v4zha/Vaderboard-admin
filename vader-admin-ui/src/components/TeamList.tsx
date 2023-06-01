import {
    List,
    ListItem,
    ListItemText,
    IconButton,
    Container,
    TextField,
} from "@mui/material";
import FileCopyIcon from "@mui/icons-material/FileCopy";
import { getTeams } from "../utils/apiUtils";
import { useEffect, useState } from "react";

const TeamList = (): JSX.Element => {
    const [teams, setTeams] = useState<Array<TeamInfo>>([]);
    const [socket, setSocket] = useState<WebSocket>();
    useEffect(() => {
        (async () => {
            const t = await getTeams();
            setTeams(t);
        })();
    }, []);
    useEffect(() => {
        const ws = new WebSocket("ws://localhost:8080/team/fts");
        ws.addEventListener("open", () => {
            console.log("ws connected.");
        });

        ws.addEventListener("message", (event) => {
            const data: Array<UserInfo> = JSON.parse(event.data);
            setTeams(data);
        });

        setSocket(ws);
        return () => {
            ws.close();
        };
    }, []);
    const handleClick = async (teamId: string) => {
        try {
            await navigator.clipboard.writeText(teamId);
            console.log("Team ID copied:", teamId);
        } catch (error) {
            console.error("Failed to copy team ID to clipboard:", error);
        }
    };
    const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
        const param = event.target.value;
        if (socket) {
            socket.send(param);
        }
    };

    return (
        <Container sx={{ marginTop: "2rem" }}>
            <TextField
                label="Search Teams"
                variant="outlined"
                fullWidth
                onChange={handleSearch}
            />
            <List>
                {teams.map((team) => (
                    <ListItem key={team.id}>
                        <ListItemText
                            primary={team.name}
                            secondary={`${team.score} - ${team.id}`}
                        />
                        <IconButton
                            onClick={() => handleClick(team.id)}
                            aria-label="Copy Team ID"
                        >
                            <FileCopyIcon />
                        </IconButton>
                    </ListItem>
                ))}
            </List>
        </Container>
    );
};

export default TeamList;
