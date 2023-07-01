import React, { lazy, Suspense } from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import { AddUser } from "./pages/AddUser";
import { AddTeamMembers } from "./pages/AddTeamMember";
import { AddTeam } from "./pages/AddTeam";

const Login = lazy(() => import("./pages/Login"));
const Home = lazy(() => import("./pages/Home"));
const Event = lazy(() => import("./pages/Event"));
const AddEvent = lazy(() => import("./pages/AddEvent"));

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
                    <Route path="/event" element={<Event />} />
                    <Route path="/event/add" element={<AddEvent />} />
                    <Route path="/user/add" element={<AddUser />} />
                    <Route path="/team/add" element={<AddTeam />} />
                    <Route
                        path="/team/member/add"
                        element={<AddTeamMembers />}
                    />
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
