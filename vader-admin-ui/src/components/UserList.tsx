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
import React, { useEffect, useState } from "react";
import { UserInfo } from "../Types";
import { apiUrl } from "../utils/apiUtils";

interface UserListProps {
    url: string;
    updateScore?: boolean;
}

interface ScoreUpdate {
    id: string;
    score: number;
}

const UserList = (props: UserListProps): JSX.Element => {
    const [users, setUsers] = useState<Array<UserInfo>>([]);
    const [socket, setSocket] = useState<WebSocket>();
    const [updatedScores, setUpdatedScores] = useState<Array<ScoreUpdate>>([]);

    useEffect(() => {
        const ws = new WebSocket(props.url);
        ws.addEventListener("open", () => {
            console.log("ws connected.");
            ws.send("");
        });

        ws.addEventListener("message", (event) => {
            const data: Array<UserInfo> = JSON.parse(event.data);
            setUsers(data);
        });

        setSocket(ws);
        return () => {
            ws.close();
        };
    }, []);

    const handleClick = async (userId: string) => {
        try {
            await navigator.clipboard.writeText(userId);
            console.log("User ID copied:", userId);
        } catch (error) {
            console.error("Failed to copy user ID to clipboard:", error);
        }
    };

    const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
        const param = event.target.value;
        if (socket) {
            socket.send(param);
        }
    };

    const handleScoreUpdate = (userId: string) => {
        const updatedScore = updatedScores.find((score) => score.id === userId);
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
        userId: string
    ) => {
        const updatedScore = Number(event.target.value);
        const updatedScoresCopy = [...updatedScores];
        const existingScoreIndex = updatedScoresCopy.findIndex(
            (score) => score.id === userId
        );

        if (existingScoreIndex !== -1) {
            updatedScoresCopy[existingScoreIndex].score = updatedScore;
        } else {
            updatedScoresCopy.push({ id: userId, score: updatedScore });
        }

        setUpdatedScores(updatedScoresCopy);
    };

    return (
        <Container sx={{ marginTop: "2rem" }}>
            <TextField
                label="Search Users"
                variant="outlined"
                fullWidth
                onChange={handleSearch}
            />
            <List>
                {users.map((user) => (
                    <ListItem key={user.id}>
                        <ListItemText
                            primary={user.name}
                            secondary={`${user.score} - ${user.id}`}
                        />
                        {props.updateScore && (
                            <div>
                                <TextField
                                    type="number"
                                    value={
                                        updatedScores.find(
                                            (score) => score.id === user.id
                                        )?.score || ""
                                    }
                                    onChange={(event) =>
                                        handleScoreChange(
                                            event as React.ChangeEvent<HTMLInputElement>,
                                            user.id
                                        )
                                    }
                                />
                                2
                                <Button
                                    onClick={() => handleScoreUpdate(user.id)}
                                    variant="outlined"
                                    size="small"
                                >
                                    Update Score
                                </Button>
                            </div>
                        )}
                        <IconButton
                            onClick={() => handleClick(user.id)}
                            aria-label="Copy User ID"
                        >
                            <FileCopyIcon />
                        </IconButton>
                    </ListItem>
                ))}
            </List>
        </Container>
    );
};

export default UserList;
