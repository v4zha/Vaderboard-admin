import {
    List,
    ListItem,
    ListItemText,
    IconButton,
    TextField,
    Container,
    Button,
} from "@mui/material";
import FileCopyIcon from "@mui/icons-material/FileCopy";
import {  useEffect, useState } from "react";
import { TeamInfo } from "../Types";
import { apiUrl } from "../utils/apiUtils";

interface TeamListProps {
    url: string;
    updateScore?: boolean;
}

interface ScoreUpdate {
    id: string;
    score: number;
}

const TeamList = (props: TeamListProps): JSX.Element => {
    const [teams, setTeams] = useState<Array<TeamInfo>>([]);
    const [socket, setSocket] = useState<WebSocket>();
    const [updatedScores, setUpdatedScores] = useState<Array<ScoreUpdate>>([]);

    useEffect(() => {
        const ws = new WebSocket(props.url);
        ws.addEventListener("open", () => {
            console.log("ws connected.");
            ws.send("");
        });

        ws.addEventListener("message", (event) => {
            const data: Array<TeamInfo> = JSON.parse(event.data);
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

    const handleScoreUpdate = (teamId: string) => {
        const updatedScore = updatedScores.find((score) => score.id === teamId);
        if (updatedScore) {
            const data = JSON.stringify(updatedScore);

            fetch(`${apiUrl}admin/score/update`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: data,
            })
                .then((response) => {
                    if (response.ok) {
                        console.log("Score updated successfully");
                    } else {
                        console.error("Failed to update score");
                    }
                })
                .catch((error) => {
                    console.error("Failed to update score:", error);
                });
        }
    };

    const handleScoreChange = (
        event: React.ChangeEvent<HTMLInputElement>,
        teamId: string
    ) => {
        const updatedScore = Number(event.target.value);
        const updatedScoresCopy = [...updatedScores];
        const existingScoreIndex = updatedScoresCopy.findIndex(
            (score) => score.id === teamId
        );

        if (existingScoreIndex !== -1) {
            updatedScoresCopy[existingScoreIndex].score = updatedScore;
        } else {
            updatedScoresCopy.push({ id: teamId, score: updatedScore });
        }

        setUpdatedScores(updatedScoresCopy);
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
                        {props.updateScore && (
                            <div>
                                <TextField
                                    type="number"
                                    value={
                                        updatedScores.find(
                                            (score) => score.id === team.id
                                        )?.score || ""
                                    }
                                    onChange={(event) =>
                                        handleScoreChange(
                                            event as React.ChangeEvent<HTMLInputElement>,
                                            team.id
                                        )
                                    }
                                />
                                <Button
                                    onClick={() => handleScoreUpdate(team.id)}
                                    variant="outlined"
                                    size="small"
                                >
                                    Update Score
                                </Button>
                            </div>
                        )}
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
