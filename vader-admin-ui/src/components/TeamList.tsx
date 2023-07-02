import {
    List,
    ListItem,
    ListItemText,
    IconButton,
    TextField,
    Container,
    Button,
    Dialog,
    DialogTitle,
    DialogContent,
    DialogActions,
} from "@mui/material";
import FileCopyIcon from "@mui/icons-material/FileCopy";
import { useEffect, useState } from "react";
import { TeamInfo } from "../Types";
import { apiUrl } from "../utils/apiUtils";

interface TeamListProps {
    url: string;
    updateScore?: boolean;
}

const TeamList = (props: TeamListProps): JSX.Element => {
    const [teams, setTeams] = useState<Array<TeamInfo>>([]);
    const [socket, setSocket] = useState<WebSocket>();
    const [selectedTeamId, setSelectedTeamId] = useState<string>("");
    const [isDialogOpen, setIsDialogOpen] = useState<boolean>(false);
    const [newScore, setNewScore] = useState<number>(0);

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
        setSelectedTeamId(teamId);
        setIsDialogOpen(true);
    };

    const handleScoreChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setNewScore(Number(event.target.value));
    };

    const handleSubmit = () => {
        const updatedScore = {
            id: selectedTeamId,
            score: newScore,
        };

        const data = JSON.stringify(updatedScore);

        fetch(`${apiUrl}/admin/score/update`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: data,
        })
            .then((response) => {
                if (response.ok) {
                    console.log("Score updated successfully");
                    if (socket) {
                        socket.send("");
                    }
                } else {
                    console.error("Failed to update score");
                }
            })
            .catch((error) => {
                console.error("Failed to update score:", error);
            });

        setIsDialogOpen(false);
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
            <Dialog open={isDialogOpen} onClose={() => setIsDialogOpen(false)}>
                <DialogTitle>Update Score</DialogTitle>
                <DialogContent>
                    <TextField
                        label="Current Score"
                        value={
                            teams.find((team) => team.id === selectedTeamId)
                                ?.score || ""
                        }
                        disabled
                    />
                    <TextField
                        label="New Score"
                        type="number"
                        value={newScore}
                        onChange={handleScoreChange}
                    />
                </DialogContent>
                <DialogActions>
                    <Button onClick={() => setIsDialogOpen(false)}>
                        Cancel
                    </Button>
                    <Button onClick={handleSubmit}>Submit</Button>
                </DialogActions>
            </Dialog>
        </Container>
    );
};

export default TeamList;
