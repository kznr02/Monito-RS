import { createBrowserRouter } from "react-router";
import FrontPage from "../FrontPage";
import SecondPage from "../SecondPage";

const routers = createBrowserRouter([
    {
        path: '/',
        element: <FrontPage />
    },
    {
        path: '/setting',
        element: <SecondPage />
    }
]);

export default routers;