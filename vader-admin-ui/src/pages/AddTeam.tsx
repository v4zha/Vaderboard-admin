import React, { useState } from "react";
import {
    Button,
    Container,
    TextField,
    Typography,
    Snackbar,
    IconButton,
} from "@mui/material";
import { apiUrl } from "../utils/apiUtils";
import { useNavigate, useLocation } from "react-router-dom";
import AddCircleIcon from "@mui/icons-material/AddCircle";
import { TeamEventOpts } from "../Types";

export const AddTeam = () => {
    const [teamName, setTeamName] = useState("");
    const [teamMembers, setTeamMembers] = useState([{ name: "" }]);
    const [showSnackbar, setShowSnackbar] = useState(false);
    const [snackbarMessage, setSnackbarMessage] = useState("");
    const navigate = useNavigate();
    const location = useLocation();

    const maxTeamSize = location.state?.maxTeamSize || 0;

    const handleFormSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        if (teamMembers.length === 0) {
            setSnackbarMessage("Please add at least one team member.");
            setShowSnackbar(true);
            return;
        }

        const url = `${apiUrl}/admin/event/team/add/with_members`;
        const data = {
            team_info: {
                name: teamName,
            },
            members: teamMembers,
        };

        try {
            const response = await fetch(url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(data),
                redirect: "follow",
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

    const handleAddMember = () => {
        if (teamMembers.length < maxTeamSize - 1) {
            setTeamMembers([...teamMembers, { name: "" }]);
        }
    };

    const handleMemberNameChange = (
        e: React.ChangeEvent<HTMLInputElement>,
        index: number
    ) => {
        const updatedMembers = [...teamMembers];
        updatedMembers[index].name = e.target.value;
        setTeamMembers(updatedMembers);
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
                    {teamMembers.map((member, index) => (
                        <div key={index}>
                            <TextField
                                label={`Member ${index + 1}`}
                                type="text"
                                variant="outlined"
                                fullWidth
                                value={member.name}
                                onChange={(e) =>
                                    handleMemberNameChange(
                                        e as React.ChangeEvent<HTMLInputElement>,
                                        index
                                    )
                                }
                            />
                        </div>
                    ))}
                    {teamMembers.length < maxTeamSize - 1 && (
                        <IconButton onClick={handleAddMember} color="primary">
                            <AddCircleIcon />
                        </IconButton>
                    )}
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
