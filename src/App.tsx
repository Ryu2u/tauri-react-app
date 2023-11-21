import './App.css'
import {BrowserRouter, Route, Routes} from "react-router-dom";
import {AdminComponent} from "./admin/AdminComponent.tsx";
import {ChatComponent} from "./admin/chat/ChatComponent.tsx";
import {LoginComponent} from "./login/LoginComponent.tsx";

function App() {

    return (
        <>
            <BrowserRouter>
                <Routes>
                    <Route path={"/admin"} element={<AdminComponent/>}>
                        <Route path={"/admin/chat"} element={<ChatComponent/>}>

                        </Route>
                    </Route>
                    <Route path={"/login"} element={<LoginComponent/>}>

                    </Route>
                </Routes>
            </BrowserRouter>
        </>
    )
}

export default App
