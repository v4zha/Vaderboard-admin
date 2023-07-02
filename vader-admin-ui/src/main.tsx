import React, { Suspense } from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import { AddUser } from "./pages/AddUser";
import { AddTeam } from "./pages/AddTeam";
import Login from "./pages/Login";
import Home from "./pages/Home";
import AddEvent from "./pages/AddEvent";
import { VaderEvent } from "./pages/Event";

const theme = createTheme({
    palette: {
        primary: {
            main: "#81A1C1",
        },
        background: {
            default: "#2E3440",
        },
        text: {
            primary: "#ECEFF4",
            secondary: "#8FBCBB",
        },
    },
});

const App = (): JSX.Element => (
    <ThemeProvider theme={theme}>
        <CssBaseline />
        <Router>
            <Suspense fallback={<div>Loading...</div>}>
                <Routes>
                    <Route path="/" element={<Login />} />
                    <Route path="/home" element={<Home />} />
                    <Route path="/event" element={<VaderEvent />} />
                    <Route path="/event/add" element={<AddEvent />} />
                    <Route path="/user/add" element={<AddUser />} />
                    <Route path="/team/add" element={<AddTeam />} />
                </Routes>
            </Suspense>
        </Router>
    </ThemeProvider>
);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);
