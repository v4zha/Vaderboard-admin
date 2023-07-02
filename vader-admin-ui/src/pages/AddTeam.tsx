import React, { useState } from "react";
import {
    Button,
    Container,
    TextField,
    Typography,
    Snackbar,
} from "@mui/material";
import { apiUrl } from "../utils/apiUtils";
import { useNavigate } from "react-router-dom";
import { TeamEventOpts } from "../Types";

export const AddTeam = () => {
    const [teamName, setTeamName] = useState("");
    const [showSnackbar, setShowSnackbar] = useState(false);
    const [snackbarMessage, setSnackbarMessage] = useState("");
    const navigate = useNavigate();

    const handleFormSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const url = `${apiUrl}/admin/event/team/add`;
        const data = {
            name: teamName,
        };
        try {
            const response = await fetch(url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(data),
            });
            if (response.ok) {
                setShowSnackbar(true);
                setSnackbarMessage("Team added successfully!");
                navigate("/event", {
                    state: {
                        opt: TeamEventOpts.TeamList,
                    },
                });
            } else {
                setShowSnackbar(true);
                setSnackbarMessage("Failed to add team");
            }
        } catch (error: any) {
            setShowSnackbar(true);
            setSnackbarMessage("Error: " + error.message);
        }
    };

    const handleSnackbarClose = () => {
        setShowSnackbar(false);
    };

    return (
        <Container maxWidth="sm">
            <div className="team-container">
                <Typography variant="h6" color="inherit" component="div">
                    Add Team
                </Typography>
                <form onSubmit={handleFormSubmit} className="form-container">
                    <TextField
                        id="team-name"
                        label="Team Name"
                        type="text"
                        variant="outlined"
                        fullWidth
                        value={teamName}
                        onChange={(e) => setTeamName(e.target.value)}
                    />
                    <Button
                        type="submit"
                        variant="contained"
                        color="primary"
                        fullWidth
                    >
                        Add Team
                    </Button>
                </form>
            </div>
            <Snackbar
                open={showSnackbar}
                autoHideDuration={3000}
                onClose={handleSnackbarClose}
                message={snackbarMessage}
            />
        </Container>
    );
};
